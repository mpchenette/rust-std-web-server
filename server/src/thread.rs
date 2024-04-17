// Type alias for a Job. A Job is represented as a Box (type) that contains a boxed (heap-allocated)
// dynamic (`dyn`) closure (`FnOnce`) that takes no arguments and returns no value.
// The closure can be sent between threads (`Send`) and does not reference any non-static (temporary) data (`'static`).
// This is the type of jobs that will be executed by the threads in the thread pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

// Type alias that allows safe, concurrent access to shared data from multiple threads. Is a RC smart pointer to a Mutex that guards a Receiver<Job>.
// An ARC wrapped (atomic reference counted - allows multiple threads to own the Mutex<Receiver<Job>> and ensures that the Receiver gets cleaned up once all Arc references are out of scope.)
// mutually exclusive (can only be used by one thread at a time using locks)
// Job receiver (...)
type ArcMutexReceiver = std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Job>>>;

// Create a Worker struct that contains a thread handle. The thread handle is a JoinHandle that is used to join the thread.
struct Worker {
    thread: std::thread::JoinHandle<()>,
}

// Implement the Worker struct
impl Worker {
    // constructor - Create a new Worker instance
    fn new(receiver: ArcMutexReceiver) -> Worker {
        // Create a new thread that will run the worker function. The worker function will receive jobs from the receiver and execute them.
        let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            // Loop to receive jobs from the receiver and execute them
            // The loop will continue to run as long as the receiver is able to receive jobs
            // The receiver is a mutex-protected channel receiver. The receiver is locked and the recv method is called to receive a job.
            //
            //                              Mutex      Receiver
            //                             ___|___        |
            //                            |       |       |
            while let Ok(job) = receiver.lock().unwrap().recv() { //handle errors here, no unwrap
                job();
            }
        });

        // Return the new Worker instance
        Worker { thread }
    }
}

pub struct Pool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

impl Pool {
    pub fn new(size: usize) -> Pool {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver: ArcMutexReceiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(Worker::new(std::sync::Arc::clone(&receiver)));
        }

        Pool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<F> = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
