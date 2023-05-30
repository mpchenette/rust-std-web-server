
// things not in std: log, regex, tls

use std::{
    io::{Read, Write},
};

struct HttpRequest {
    method: String,
    url: String,
    version: String,
}

const DOCKER: bool = false;
const SITE_PATH: &str = "site/";

// returns a populated HttpRequest struct if valid, shuts down the TcpStream and panics the current thread if not
fn is_valid_http(mut tcp_stream: &std::net::TcpStream) -> HttpRequest {
    println!("\nIS_VALID_HTTP\n-------------");
    let mut buffer: [u8; 5000] = [0_u8; 5000];

    // Read incoming data into a "typeless" buffer
    match tcp_stream.read(&mut buffer) {
        Ok(size) => {

            // "Cast" typeless data into a &str
            let str_to_split: &str = match std::str::from_utf8(&buffer[0..size]) {
                Ok(str) => str,
                Err(e) => panic!("Invalid UTF-8: {}", e)
            };

            // Split new &str on whitespace. Would prefer regex here for validation and ease of use purposes.
            let mut request_items: std::str::SplitWhitespace = str_to_split.split_whitespace();

            // Construct HttpRequest
            let http_request: HttpRequest = HttpRequest {
                method: request_items.next().unwrap().to_string(),
                url: request_items.next().unwrap().to_string(),
                version: request_items.next().unwrap().to_string()
            };
            println!("returning:\n   method: {}\n   url: {}\n   version: {}\n", http_request.method, http_request.url, http_request.version);

            // Return HttpRequest
            http_request
        }
        Err(e) => {
            // Shutdown the current TcpStream (i.e., connection)
            match tcp_stream.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {},
                Err(e) => println!("TcpStream Shutdown Error: {}", e)
            };

            // Panic the current thread
            panic!("Invalid HTTP Request. Shutting down the TcpStream. Error: {}", e);
        }
    }
}

fn handle_tcp_stream(mut tcp_stream: std::net::TcpStream) {
    println!("\nHANDLE_TCP_STREAM\n-----------------");

    // Validate that the incoming data is a valid HTTP request
    let http_request: HttpRequest = is_valid_http(&tcp_stream);

    // Load the requested page's contents into a "typeless" vector
    let file_path: String = if http_request.url.eq("/") { String::from("index.html") } else { http_request.url };
    let file_contents: Vec<u8> = match std::fs::read(format!("{}{}", SITE_PATH, &file_path)) {
        Ok(v) => v,
        Err(_) => std::fs::read(format!("{}{}", SITE_PATH, "404.html")).unwrap()
    };

    // Write the loaded contents to the TcpStream (i.e., connection)
    match tcp_stream.write(&file_contents) {
        Ok(size) => println!("TcpStream Write Success: {} (of size {} bytes) written to {}", file_path, size, tcp_stream.local_addr().unwrap()),
        Err(e) => println!("TcpStream Write Error: {}", e)
    };
}

fn main() {
    let addr: &str = if DOCKER { "0.0.0.0:80" } else { "127.0.0.1:80" }; // https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    let tcp_listener: std::net::TcpListener = match std::net::TcpListener::bind(addr){
        Ok(tcp_listener) => tcp_listener,
        Err(e) => panic!("Unable to bind to {}. Error: {}", addr, e),
    };
    println!("Listening on {}", tcp_listener.local_addr().unwrap());
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                // https://doc.rust-lang.org/std/thread/index.html#spawning-a-thread
                let thread_join_handle: std::thread::JoinHandle<()> = std::thread::spawn(move || handle_tcp_stream(tcp_stream));
                let join_result: Result<(), Box<dyn std::any::Any + Send>> = thread_join_handle.join();

                // https://doc.rust-lang.org/std/thread/type.Result.html#examples
                match join_result {
                    Ok(_) => println!("join succeeded"),
                    Err(e) => std::panic::resume_unwind(e)
                }
            }
            Err(e) => {
                println!("TcpStream Error: {}", e)
            }
        }
    }
    std::mem::drop(tcp_listener);
}
