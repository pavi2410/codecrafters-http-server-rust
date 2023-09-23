// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_stream(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(stream: &mut TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
}