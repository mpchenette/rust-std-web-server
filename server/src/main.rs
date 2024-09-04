// main.rs

mod tcp;
mod thread;

// const DOCKER: bool = true;

// Create a TcpListener (on the server) to handle TcpStreams (i.e., connections from clients)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("LOG (MAIN): Starting server");

    // // Determine which IPv4 address we will attempt to bind to - https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    // let ipv4_address: &str = if DOCKER {
    //     "0.0.0.0:8000"
    // } else {
    //     "127.0.0.1:8000"
    // }; // 08AUG2024: is this needed? on my mac, when DOCKER = true, I can still connect to 0.0.0.0:8000 from localhost:8000

    // TODO: should we only allow IPv6 or allow IPv4 too?
    let ipv6_addr: std::net::Ipv6Addr = std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let port: u16 = 8000;
    let ipv6_socket_addr: std::net::SocketAddrV6 = std::net::SocketAddrV6::new(ipv6_addr, port, 0, 0);

    // Create the TcpListener by binding to the determined socket address (i.e., ip + port)
    let tcp_listener: std::net::TcpListener = std::net::TcpListener::bind(ipv6_socket_addr)?; // Anything that implements std::net::ToSocketAddrs can be provided here. This includes a &str, a full-blown Socket Addr like I do here, or even a (ipv6addr, port). See, https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html for more info.

    // Log the address that the TcpListener is listening on
    match tcp_listener.local_addr() {
        Ok(local_addr) => println!("LOG (MAIN): Server is listening on {}", local_addr),
        Err(e) => println!("WARNING (MAIN): Failed to log the local address: {}", e),
    }

    // Create a thread pool to handle incoming TcpStreams (i.e., connections from clients)
    let pool: thread::Pool = thread::Pool::new(4); // When idle, threads seem to consume, on average, ~40 kB of memory each

    // Handle the TcpStream (connection) of each client who connects to the server (via the TcpListener)
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {
                // Log the address of the connected TcpStream (client)
                match tcp_stream.local_addr() {
                    Ok(local_addr) => {
                        println!("\nLOG (MAIN): New TcpStream Received ({})", local_addr)
                    }
                    Err(e) => println!("WARNING (MAIN): Failed to log the local address: {}", e),
                }

                // Handle the TcpStream (connection) using a thread from the thread pool
                pool.execute(/* move? */ || tcp::handle_tcp_stream(tcp_stream));
            }
            Err(e) => {
                println!("ERROR (MAIN): TcpStream Error: {}", e)
            }
        }
    }
    std::mem::drop(tcp_listener);

    Ok(())
}

// pub const HTTP_VERSIONS_REFERENCES: [&[u8; 8]; 8] = [
//     b"HTTP/1.0",
//     b"HTTP/1.1",
//     b"HTTP/1.2",
//     b"HTTP/1.3",
//     b"HTTP/1.4",
//     b"HTTP/1.5",
//     b"HTTP/2.0",
//     b"HTTP/3.0",
// ];
// pub const HTTP_VERSIONS_VALUES: [[u8; 8]; 8] = [
//     *b"HTTP/1.0",
//     *b"HTTP/1.1",
//     *b"HTTP/1.2",
//     *b"HTTP/1.3",
//     *b"HTTP/1.4",
//     *b"HTTP/1.5",
//     *b"HTTP/2.0",
//     *b"HTTP/3.0",
// ];

// fn benchmark_slices() {
//     let mut total_duration = std::time::Duration::new(0, 0);
//     for _ in 0..100 {
//         let start = std::time::Instant::now();
//         for _ in 0..1_000_000 {
//             for method in HTTP_VERSIONS_REFERENCES.iter() {
//                 if *method == b"HTTP/2.0" {
//                     std::hint::black_box(method);
//                 }
//             }
//         }
//         total_duration += start.elapsed();
//     }
//     println!("Average time taken for slices: {:?}", total_duration / 10);
// }

// fn benchmark_arrays() {
//     let mut total_duration = std::time::Duration::new(0, 0);
//     for _ in 0..100 {
//         let start = std::time::Instant::now();
//         for _ in 0..1_000_000 {
//             for version in HTTP_VERSIONS_VALUES.iter() {
//                 if version == b"HTTP/2.0" {
//                     std::hint::black_box(version);
//                 }
//             }
//         }
//         total_duration += start.elapsed();
//     }
//     println!("Average time taken for arrays: {:?}", total_duration / 10);
// }

// fn main() {
//     benchmark_slices();
//     benchmark_arrays();
// }