use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

const MAX_MSG_SIZE: usize = 1000;

enum Message {
    ConnectionEstablished(TcpStream),
    ConnectionClosed(TcpStream),
    NewMessage(String),
}

fn main() -> Result<(), Box<dyn Error>> {
    // Start a listening on a local port.
    let listener = TcpListener::bind("127.0.0.1:1234").unwrap();

    // Create a channel for message passing between clients and server.
    let (sender, receiver) = mpsc::channel();

    // Start a thread as server.
    // use std::thread;
    let _server = thread::spawn(|| {
        run_server(receiver);
    });

    // Accept connections, for each connection, start a separate client thread
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                let tx = sender.clone();
                // Start a thread as client.
                let _client = thread::spawn(|| {
                    run_client(stream, tx);
                });
            }
            Err(e) => {
                eprint!("ERROR: accepting connections from a client {e}");
                break;
            }
        }
    }

    Ok(())
}

fn run_server(rx: mpsc::Receiver<Message>) {
    println!("The server thread started.");

    // Handle incoming Messages.
    loop {
        let msg = rx.recv().expect("should receive a message.");
        match msg {
            Message::ConnectionEstablished(_) => {
                println!("A connection from a client is established.")
            }
            Message::ConnectionClosed(_) => {
                println!("A connection from a client is closed.")
            }
            Message::NewMessage(client_msg) => {
                println!("Received a message: {}", client_msg);
            }
        }
    }
}

fn run_client(mut stream: TcpStream, tx: mpsc::Sender<Message>) {
    // Accepting Inputs
    loop {
        let mut buf = [0; MAX_MSG_SIZE];
        let read_res = stream.read(&mut buf);
        match read_res {
            Ok(_) => {
                let content_res = String::from_utf8(buf.to_vec());
                match content_res {
                    Ok(c) => {
                        let _send_res = tx.send(Message::NewMessage(c));
                    }
                    Err(e) => {
                        eprintln!("ERROR: reading inputs, closing the connection. {e}");
                        let _send_res = tx.send(Message::ConnectionClosed(stream));
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("ERROR: reading inputs, closing the connection. {e}");
                let send_res = tx.send(Message::ConnectionClosed(stream));
                match send_res {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "ERROR: failed sending connection closed message back to the server. {e}"
                        );
                        break;
                    }
                }
                break;
            }
        }
    }
}
