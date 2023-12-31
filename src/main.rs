// Uncomment this block to pass the first stage
use std::{
    collections::HashMap,
    env::args,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
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

    let first_line = lines.by_ref()
    .take(1)
    .collect::<String>();

    let mut parts = first_line.split_whitespace();

    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap();

    let headers = lines.by_ref()
        .take_while(|l| !l.is_empty())
        .filter_map(|l| l.split_once(": "))
        .collect::<HashMap<_, _>>();

    let body = lines.by_ref()
        .collect::<Vec<_>>()
        .join("\n");

    if path == "/" {
        ok_response(stream);
    } else if path.starts_with("/echo") {
        text_response(stream, path.strip_prefix("/echo/").unwrap());
    } else if path == "/user-agent" {
        text_response(stream, headers.get("User-Agent").unwrap());
    } else if path.starts_with("/files") {
        match method {
            "GET" => {
                let file_name = path.strip_prefix("/files/").unwrap();

                let directory = args().nth(2).unwrap();

                let file_path = format!("{}/{}", directory, file_name);

                match std::fs::read(file_path) {
                    Ok(file_contents) => octet_stream_response(stream, &file_contents),
                    Err(_) => not_found_response(stream),
                }
            }
            "POST" => {
                let file_name = path.strip_prefix("/files/").unwrap();

                let directory = args().nth(2).unwrap();

                let file_path = format!("{}/{}", directory, file_name);

                let content_size: usize = headers.get("Content-Length").unwrap().parse().unwrap();
                std::fs::write(file_path, &body[..content_size]).unwrap();

                stream.write(b"HTTP/1.1 201 Created\r\n\r\n").unwrap();
            }
            _ => not_found_response(stream),
        }
    } else {
        not_found_response(stream);
    }
}

fn ok_response(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
}

fn not_found_response(mut stream: TcpStream) {
    stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n").unwrap();
}

fn text_response(mut stream: TcpStream, text: &str) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream.write(b"Content-Type: text/plain\r\n").unwrap();
    write!(stream, "Content-Length: {}\r\n\r\n", text.len()).unwrap();
    stream.write(text.as_bytes()).unwrap();
}

fn octet_stream_response(mut stream: TcpStream, bytes: &[u8]) {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap();
    stream
        .write(b"Content-Type: application/octet-stream\r\n")
        .unwrap();
    write!(stream, "Content-Length: {}\r\n\r\n", bytes.len()).unwrap();
    stream.write(bytes).unwrap();
}
