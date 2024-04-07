// tcp.rs

mod http; // the reason for the tcp folder: https://doc.rust-lang.org/rust-by-example/mod/split.html
const SITE_PATH: &str = "site/";

use std::io::{Read, Write};

pub fn handle_tcp_stream(mut tcp_stream: std::net::TcpStream) {
    let tcp_stream_vec_u8: Vec<u8> = read_tcp_stream_to_vec_u8(&tcp_stream);
    let http_request: http::HttpRequest =
        http::construct_http_request_from_vec_u8(tcp_stream_vec_u8);

    // Load the requested page's contents into a "typeless" vector
    let file_path: String = if http_request.url.eq("/") {
        String::from("index.html")
    } else {
        http_request.url
    };
    let file_contents: Vec<u8> = match std::fs::read(format!("{}{}", SITE_PATH, &file_path)) {
        Ok(v) => v,
        Err(_) => std::fs::read(format!("{}{}", SITE_PATH, "404.html")).unwrap(),
    };

    // Write the loaded contents to the TcpStream (i.e., connection)
    match tcp_stream.write(&file_contents) {
        Ok(size) => println!(
            "LOG (HANDLE_TCP_STREAM): TcpStream Write Success: {} (of size {} bytes) written to {}",
            file_path,
            size,
            tcp_stream.local_addr().unwrap()
        ),
        Err(e) => println!("ERROR (HANDLE_TCP_STREAM): TcpStream Write Error: {}", e),
    };
}

/// returns a "typeless" vector containing the contents of the given TcpStream (i.e., connection)
///
/// # Arguments
///
/// * `tcp_stream` - The TcpStream to read the contents of
///
/// # Returns
///
/// A Vec<u8> of the TcpStream contentsS
///
/// # Examples
///
/// ```
/// let tcp_stream_vec_u8: Vec<u8> = read_tcp_stream_to_vec_u8(&tcp_stream);
/// ```
pub fn read_tcp_stream_to_vec_u8(mut tcp_stream: &std::net::TcpStream) -> Vec<u8> {
    let mut vec_buffer: Vec<u8> = Vec::new();

    // Read incoming data into a "typeless" vector
    let mut buffer: [u8; 1024] = [0; 1024]; // Adjust chunk size as needed
    match tcp_stream.read(&mut buffer) {
        Ok(0) => {} // Connection closed by the client
        Ok(bytes_read) => {
            println!(
                "LOG (READ_TCP_STREAM_TO_VEC_U8): Read {} bytes from the TcpStream",
                bytes_read
            );
            vec_buffer.extend_from_slice(&buffer[..bytes_read]);
        }
        Err(e) => {
            // Shutdown the current TcpStream (i.e., connection)
            match tcp_stream.shutdown(std::net::Shutdown::Both) {
                Ok(_) => {}
                Err(e) => println!(
                    "ERROR (READ_TCP_STREAM_TO_VEC_U8): TcpStream Shutdown Error: {}",
                    e
                ),
            };

            // Panic the current thread
            panic!(
                "ERROR (READ_TCP_STREAM_TO_VEC_U8): Invalid HTTP Request. Shutting down the TcpStream. Error: {}",
                e
            );
        }
    }
    // println!(
    //     "Received {} bytes: {}",
    //     vec_buffer.len(),
    //     String::from_utf8_lossy(&vec_buffer)
    // );

    vec_buffer
}
