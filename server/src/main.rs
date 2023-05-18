use std::fs;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str;
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0 as u8; 5000];
    match stream.read(&mut data) {
        Ok(size) => {
            let s = match str::from_utf8(&data[0..size]) {
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
            };
            let mut toks = s.split(' ').fuse();
            let first = toks.next();
            let second = toks.next();

            println!("{:?}, {:?}", first, second);
            match second {
                Some("/") => stream
                    .write(&(fs::read("site/index.html").unwrap()))
                    .unwrap(),
                Some("/abc.png") => stream.write(&(fs::read("site/abc.png").unwrap())).unwrap(),
                Some("/favicon.ico") => stream
                    .write(&(fs::read("site/favicon.ico").unwrap()))
                    .unwrap(),
                _ => usize::MIN,
            };
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
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
