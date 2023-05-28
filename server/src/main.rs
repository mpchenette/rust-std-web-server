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
    let req: Http = is_valid_http(&stream);
    let mut url: String = String::from("site"); //String::from("/index.html"); // is this (and the below) the best way to do this?
    if req.request_url.eq("/") {
        url.push_str("/index.html");
    }
    else {
        url.push_str(&(req.request_url)); // not sure if it's faster/more efficint to set this in an else or just set it initially then overwrite if index
    }
    let file_contents: Vec<u8> = match std::fs::read(&url) {
        Ok(v) => v,
        Err(_) => std::fs::read("site/404.html").unwrap(),
    };
    stream.write(&file_contents).unwrap();
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
