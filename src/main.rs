use std::error::Error;
use std::net::{TcpListener, TcpStream};

fn main() -> Result<(), Box<dyn Error>> {
    // Start a thread to listen on a local port.
    let _listener = TcpListener::bind("127.0.0.1:1234");
    // Connect a stream to the local port.
    let stream =
        TcpStream::connect("127.0.0.1:1234").expect("should be able to connect to the local port.");
    println!(
        "Connected! Local IP is {}",
        stream.local_addr().unwrap().ip()
    );
    Ok(())
}
