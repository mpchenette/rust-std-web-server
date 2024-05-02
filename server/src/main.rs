// main.rs

mod tcp;
mod thread;

const DOCKER: bool = true;

// fn main() - Create a TcpListener to handle TcpStreams (i.e., connections) from clients
fn main() {
    println!("LOG (MAIN): Starting server");

    // Determine which IPv4 address we will attempt to bind to - https://stackoverflow.com/questions/66725777/how-to-connect-to-rust-server-app-that-runs-on-docker
    let address: &str = if DOCKER { "0.0.0.0:80" } else { "127.0.0.1:80" };

    // Create the TcpListener by binding to the determined address
    let tcp_listener: std::net::TcpListener = match std::net::TcpListener::bind(address) {
        Ok(tcp_listener) => tcp_listener,
        Err(e) => panic!("ERROR (MAIN): Unable to bind to {}. Error: {}", address, e),
    };

    // Log the address that the TcpListener is listening on
    match tcp_listener.local_addr() {
        Ok(local_addr) => println!("LOG (MAIN): Server is listening on {}", local_addr),
        Err(e) => println!("WARNING (MAIN): Failed to log the local address: {}", e),
    }

    // Create a thread pool to handle the TcpStreams (i.e., connections) from clients
    let pool: thread::Pool = thread::Pool::new(4); // When idle, threads seem to consume, on average, ~40 kB of memory each

    // Handle the TcpStream (connection) of each client who connects to the server (via the TcpListener)
    for tcp_stream in tcp_listener.incoming() {
        match tcp_stream {
            Ok(tcp_stream) => {

                // Log the address of the connected TcpStream (client)
                match tcp_stream.local_addr() {
                    Ok(local_addr) => println!("\nLOG (MAIN): New TcpStream Received ({})", local_addr),
                    Err(e) => println!("WARNING (MAIN): Failed to log the local address: {}", e),
                }

                // Handle the TcpStream (connection) using a thread from the thread pool
                pool.execute(/* move? */|| tcp::handle_tcp_stream(tcp_stream));
            }
            Err(e) => {
                println!("ERROR (MAIN): TcpStream Error: {}", e)
            }
        }
    }
    std::mem::drop(tcp_listener);
}
