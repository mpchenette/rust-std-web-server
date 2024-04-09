mod pool;

fn handle_connection() {
    // Simulate handling a connection
    let _x: u32 = (0..10000).sum();
}

fn main() {
    let num_tasks: usize = 100;

    // Benchmark thread pool
    let pool: pool::ThreadPool = pool::ThreadPool::new(4);
    let start: std::time::Instant = std::time::Instant::now();
    for _ in 0..num_tasks {
        pool.execute(|| handle_connection());
    }
    let duration: std::time::Duration = start.elapsed();
    println!("Time elapsed using thread pool is: {:?}", duration);

    // Benchmark creating a new thread for each task
    let start: std::time::Instant = std::time::Instant::now();
    let mut handles = Vec::with_capacity(num_tasks);
    for _ in 0..num_tasks {
        let handle = std::thread::spawn(|| handle_connection());
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let duration = start.elapsed();
    println!(
        "Time elapsed creating a new thread for each task is: {:?}",
        duration
    );
}
