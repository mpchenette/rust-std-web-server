use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

// fn handle_connection(mut stream: TcpStream) {
//     // println!("pwd: {:?}", std::env::current_dir());
//     // let paths = fs::read_dir("./").unwrap();
//     // for path in paths {
//     //     println!("Name: {}", path.unwrap().path().display())
//     // }
//     let header: String = "HTTP/1.1 200 OK Content-Type: text/html; charset=utf-8".to_string();
//     let webpage: String = String::from_utf8_lossy(&fs::read("./index.html").unwrap())
//         .parse()
//         .unwrap();
//     // stream
//     //     .write(b"HTTP/1.1 200 OK Content-Type: text/html; charset=utf-8")
//     //     .unwrap();

//     stream.write(header.as_bytes()).unwrap();
//     stream.write(webpage.as_bytes()).unwrap();

//     // stream.write(&(fs::read("../index.html").unwrap())).unwrap();
// }

fn handle_connection(mut stream: TcpStream) {
    // let webpage: String = String::from_utf8_lossy(&fs::read("index.html").unwrap())
    //     .parse()
    //     .unwrap();
    let mut data = [0 as u8; 5000];
    println!("reading stream");

    while match stream.read(&mut data) {
        Ok(size) => {
            let s = match str::from_utf8(&data[0..size]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            println!("{}", s);

            // echo everything!
            // stream.write(webpage.as_bytes()).unwrap();
            stream.write(&(fs::read("index.html").unwrap())).unwrap();

            false
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:80").unwrap(); // https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    println!("listening on port 80");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_connection(stream));
            }
            Err(e) => {
                println!("connection failed. error: {}", e);
            }
        }
    }
    std::mem::drop(listener); // core::mem vs. std::mem?
}
