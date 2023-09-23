// Uncomment this block to pass the first stage
use std::{net::{TcpListener, TcpStream}, io::{Write, Read}};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_stream(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 4096];

    stream.read(&mut buf).unwrap();

    let request = String::from_utf8_lossy(&buf);

    let first_line = request.lines().next().unwrap();

    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap();

    if path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    } else if path.starts_with("/echo") {
        stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
        stream.write(b"Content-Type: text/plain\r\n").unwrap();

        match path.strip_prefix("/echo/") {
            Some(echo_string) => {
                write!(stream, "Content-Length: {}\r\n\r\n", echo_string.len()).unwrap();
                stream.write(echo_string.as_bytes()).unwrap();
            }
            None => {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
            }
        }
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
    }
}