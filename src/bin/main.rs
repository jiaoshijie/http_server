use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;

use live_server::ThreadPool;

// gloable const variable
const GET_REQUEST: &[u8] = b"GET";
const STATUS_200: &str = "HTTP/1.1 200 OK";
const STATUS_404: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let b_string = String::from_utf8_lossy(&buffer).to_string();
    let (status_line, requets_path, is_file) = if buffer.starts_with(GET_REQUEST) {
        handle_get(&b_string)
    } else {
        eprintln!("{}", b_string);
        (STATUS_404, String::from("/404.html"), true)
    };
    let reponse;
    if is_file {
        let requets_path = PathBuf::from(&format!("{}{}", ".", requets_path));
        // contents = fs::read_to_string(requets_path).unwrap();
        let contents = fs::read(requets_path).unwrap();
        reponse = format!(
            "{}\r\nContent-Length: {}\r\n\r\n",
            status_line,
            contents.len(),
        );
        stream.write(reponse.as_bytes()).unwrap();
        stream.write(&contents[..]).unwrap();
    } else {
        let dir_list = handle_dir(requets_path);
        reponse = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            dir_list.len(),
            dir_list
        );
        stream.write(reponse.as_bytes()).unwrap();
    }
    stream.flush().unwrap();
}

fn handle_get(request_info: &String) -> (&'static str, String, bool) {
    let path_str = request_info
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap();
    let mut path_str = handle_special_chars(path_str);
    let relative_path = PathBuf::from(&format!("{}{}", ".", path_str));
    let is_file;
    if relative_path.exists() && relative_path.is_file() {
        is_file = true;
    } else if relative_path.exists() {
        (path_str, is_file) = have_index_html(path_str);
    } else {
        return (STATUS_404, String::from("/404.html"), true);
    }
    (STATUS_200, path_str, is_file)
}

fn handle_special_chars(path_str: &str) -> String {
    // TODO: make a map to replace sepcial chars
    path_str.replace("%20", " ")
}

fn have_index_html(path: String) -> (String, bool) {
    let relative_path = PathBuf::from(&format!("{}{}", ".", path));
    let ipath = relative_path.join("index.html");
    if ipath.exists() && ipath.is_file() {
        (format!("{}/{}", path, "index.html"), true)
    } else {
        // TODO: maybe path.is_dir()
        let f;
        if path.ends_with('/') {
            f = "";
        } else {
            f = "/";
        }
        (format!("{}{}", path, f), false)
    }
}

fn handle_dir(path: String) -> String {
    let mut list_dir = Vec::new();
    let relative_path = PathBuf::from(&format!("{}{}", ".", path));
    for entry in relative_path.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            let f;
            if entry.path().is_file() {
                f = "";
            } else {
                f = "/";
            }
            list_dir.push(format!(
                "<a href=\"{}{}{}\">{}</a><br>",
                path,
                entry.path().file_name().unwrap().to_str().unwrap(),
                f,
                entry.path().file_name().unwrap().to_str().unwrap()
            ));
        }
    }
    // TODO: dir style
    format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"> <title>{}</title></head><body><p>list:<br>{}</p></body></html>", relative_path.file_name().unwrap().to_str().unwrap(), list_dir.concat())
}
