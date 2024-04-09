// Type alias for a Job. A Job is represented as a Box that contains a 
// dynamic (`dyn`) closure (`FnOnce`) that takes no arguments and returns no value.
// The closure can be sent between threads (`Send`) and does not reference any non-static data (`'static`).

// This line is defining a type alias Job for a boxed (heap-allocated) dynamic closure that can be sent
// between threads and doesn't reference any non-static (temporary) data. This is the type of jobs that
// will be executed by the threads in the thread pool.

// This is a type alias, meaning we are not creating a new type, but rather giving a new name to an existing type. - mchenette
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    thread: std::thread::JoinHandle<()>,
}

impl Worker {
    fn new(receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker {
        let thread: std::thread::JoinHandle<()> = std::thread::spawn(move || {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                job();
            }
        });

        Worker { thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for _ in 0..size {
            workers.push(Worker::new(std::sync::Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job: Box<F> = Box::new(f);
        self.sender.send(job).unwrap();
    }
}