use std::error::Error;
use std::net::TcpListener;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    // Start a listening on a local port.
    let _listener = TcpListener::bind("127.0.0.1:1234");

    // Start a thread as server.
    // use std::thread;
    let _server = thread::spawn(|| {
        run_server();
    });

    // Start a thread as client.
    let _client = thread::spawn(|| {
        run_client();
    });

    // Note: this is a long running application.
    // TODO - implement graceful shutdown.

    // Connect a stream to the local port.
    // let stream =
    //     TcpStream::connect("127.0.0.1:1234").expect("should be able to connect to the local port.");
    // println!(
    //     "Connected! Local IP is {}",
    //     stream.local_addr().unwrap().ip()
    // );
    Ok(())
}

fn run_server() {
    println!("The server started.")
}

fn run_client() {
    println!("The client started.")
}
