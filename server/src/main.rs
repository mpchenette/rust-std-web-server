use std::{
    io::{Read, Write},
    str::SplitWhitespace,
};

struct Http {
    request_method: String,
    request_url: String,
    request_version: String,
}

// Returns an http struct if valid, null if not
// actually I think panic if invalid. that way we only accept valid http requests
fn is_valid_http(mut request: &std::net::TcpStream) -> Http {
    println!("\nIS_VALID_HTTP\n-------------");
    // let mut request_as_string: String = String::new();
    // println!("ras created");
    // match request.read_to_string(&mut request_as_string) {
    //     Ok(v) => v,
    //     Err(_) => 0,
    // };

    let mut data: [u8; 5000] = [0 as u8; 5000];
    // let s: &str = "";
    match request.read(&mut data) {
        Ok(size) => {
            let s: &str = match std::str::from_utf8(&data[0..size]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let mut request_items: SplitWhitespace = s.split_whitespace();
            let req: Http = Http {
                request_method: request_items.next().unwrap().to_string(),
                request_url: request_items.next().unwrap().to_string(),
                request_version: request_items.next().unwrap().to_string(),
            };
            return req;
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                request.peer_addr().unwrap()
            );
            request.shutdown(std::net::Shutdown::Both).unwrap();
        }
    }

    return Http {
        request_method: String::from("a"),
        request_url: String::from("b"),
        request_version: String::from("c"),
    };
}

fn handle_tcp_stream(mut stream: std::net::TcpStream) {
    println!("\nHANDLE_TCP_STREAM\n----------------");
    let req: Http = is_valid_http(&stream);
    println!("method: {}", req.request_method);
    println!("url: {}", req.request_url);
    println!("version: {}", req.request_version);
    let _ind = String::from("/");
    let _abc = String::from("/abc.png");
    let _css = String::from("/styles/style.css");
    let _ico = String::from("/favicon.ico");
    if req.request_url == "/" {

    } else if req.request_url == "/" {

    }
    // match req.request_url {
    //     "/" => stream
    //         .write(&(std::fs::read("site/index.html").unwrap()))
    //         .unwrap(),
    //     Http {
    //         request_url: _abc, ..
    //     } => stream
    //         .write(&(std::fs::read("site/abc.png").unwrap()))
    //         .unwrap(),
    //     Http {
    //         request_url: _css, ..
    //     } => stream
    //         .write(&(std::fs::read("site/styles/style.css").unwrap()))
    //         .unwrap(),
    //     Http {
    //         request_url: _ico, ..
    //     } => stream
    //         .write(&(std::fs::read("site/favicon.ico").unwrap()))
    //         .unwrap(),
    //     _ => stream
    //         .write(&(std::fs::read("site/404.html").unwrap()))
    //         .unwrap(),
    // };
}

fn main() {
    let tcp_listener: std::net::TcpListener = std::net::TcpListener::bind("127.0.0.1:80").unwrap(); // https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    println!("Listening on {}", tcp_listener.local_addr().unwrap());
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                // https://doc.rust-lang.org/std/thread/index.html#spawning-a-thread
                let thread_join_handle = std::thread::spawn(move || handle_tcp_stream(tcp_stream));
                let res = thread_join_handle.join();

                // https://doc.rust-lang.org/std/thread/type.Result.html#examples
                match res {
                    Ok(_) => println!("spawn succeeded"),
                    Err(e) => std::panic::resume_unwind(e),
                }
            }
            Err(e) => {
                println!("TcpStream Error: {}", e);
            }
        }
    }
    std::mem::drop(tcp_listener); // TODO: core::mem vs. std::mem?
}
