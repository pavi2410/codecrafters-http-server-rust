// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    io::{Read, Write},
    net::{TcpListener, TcpStream}, thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //
    for stream in listener.incoming() {
        thread::spawn(|| handle_stream(stream.unwrap()));
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buf = [0; 4096];

    stream.read(&mut buf).unwrap();

    let request = String::from_utf8_lossy(&buf);

    let mut lines = request.lines();

    let first_line = lines.next().unwrap();

    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap();

    let headers = lines
        .skip(1)
        .take_while(|line| !line.is_empty())
        .filter_map(|l| l.split_once(": "))
        .collect::<HashMap<&str, &str>>();

    if path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    } else if path.starts_with("/echo") {
        text_response(stream, path.strip_prefix("/echo/").unwrap());
    } else if path == "/user-agent" {
        text_response(stream, headers.get("User-Agent").unwrap());
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
    }
}

fn text_response(mut stream: TcpStream, text: &str) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.write(b"Content-Type: text/plain\r\n").unwrap();
    write!(stream, "Content-Length: {}\r\n\r\n", text.len()).unwrap();
    stream.write(text.as_bytes()).unwrap();
}
