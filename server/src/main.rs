// things not in std: log, regex, tls
// https://doc.rust-lang.org/std/macro.dbg.html

mod tcp;

const DOCKER: bool = false;

fn main() {
    let addr: &str = if DOCKER { "0.0.0.0:80" } else { "127.0.0.1:80" }; // https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    let tcp_listener: std::net::TcpListener = match std::net::TcpListener::bind(addr) {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => panic!("Unable to bind to {}. Error: {}", addr, e),
    };
    println!("Listening on {}", tcp_listener.local_addr().unwrap());
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                // https://doc.rust-lang.org/std/thread/index.html#spawning-a-thread
                let thread_join_handle: std::thread::JoinHandle<()> =
                    std::thread::spawn(move || tcp::handle_tcp_stream(tcp_stream));
                let join_result: Result<(), Box<dyn std::any::Any + Send>> =
                    thread_join_handle.join();

                // https://doc.rust-lang.org/std/thread/type.Result.html#examples
                match join_result {
                    Ok(_) => println!("join succeeded"),
                    Err(e) => std::panic::resume_unwind(e),
                }
            }
            Err(e) => {
                println!("TcpStream Error: {}", e)
            }
        }
    }
    std::mem::drop(tcp_listener);
}
