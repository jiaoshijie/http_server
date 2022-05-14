use std::io::prelude::*;
use std::fs;
// use std::thread;
// use std::time::Duration;
use std::net::{TcpListener, TcpStream};

use live_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("10.61.19.236:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "./hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "./404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let reponse = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
        );

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    stream.write(reponse.as_bytes()).unwrap();
    stream.flush().unwrap();
}
