use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::net::{TcpListener, TcpStream};
// use std::process;  // for exit
use std::env;

use live_server::ThreadPool;

// gloable const variable
const GET_REQUEST: &[u8] = b"GET";
const STATUS_200: &str = "HTTP/1.1 200 OK";
const STATUS_404: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    let listener = TcpListener::bind("10.61.19.236:7878").unwrap();
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


    let (status_line, filename, isfile) = if buffer.starts_with(GET_REQUEST) {
        // (STATUS_200, "./hello.html")
        // println!("{}", b_string);
        handle_get(&b_string)
    } else {  // if the head isn't `GET_REQUEST` print the client request info and return 404.html
        eprintln!("{}", b_string);
        (STATUS_404, PathBuf::from("./404.html"), true)
    };
    let contents;

    if isfile {
        contents = fs::read_to_string(filename).unwrap();
    } else {
        // TODO: handle dir
        // panic!("this is a dir!!!");
        contents = handle_dir(&filename);
    }
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

fn handle_get(req_info: &String) -> (&'static str, PathBuf, bool) {
    // TODO: don't handle spaces when file name has.
    let file_name = req_info.lines().collect::<Vec<&str>>()[0].split(" ").collect::<Vec<&str>>()[1];
    // TODO: why can't use `cwd.join()`?
    let cwd = env::current_dir().expect("runtime error: cwd error").to_str().unwrap().to_string();
    // TODO: check if is absolute path or relative path
    let mut full_path = path_join(&cwd, file_name);
    // println!("{}", full_path.display());
    let isfile;
    if is_exist(&full_path) && is_file(&full_path) {  // is file
        isfile = true
    } else if is_exist(&full_path) {  // is dir
        (full_path, isfile) = have_index_html(full_path.to_str().unwrap());
    } else {
        return (STATUS_404, PathBuf::from("./404.html"), true);
    }

    (STATUS_200, full_path, isfile)
}

fn is_exist(file: &PathBuf) -> bool {
    file.exists()
}

fn is_file(file: &PathBuf) -> bool {
    file.is_file()
}

fn path_join(dir: &str, file: &str) -> PathBuf {
    PathBuf::from(&format!("{}{}", dir, file))
}

fn have_index_html(dir: &str) -> (PathBuf, bool) {
    let full_path = path_join(dir, "index.html");
    if is_exist(&full_path) && is_file(&full_path) {
        (full_path, true)
    } else {
        (path_join(dir, "/"), false)  // TODO
    }
}

fn handle_dir(dirname: &PathBuf) -> String {
    // TODO: `.` `..`
    let mut list_dir = String::new();
    for entry in dirname.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            list_dir = format!("{}<a href=\"{}\">{}</a><br>",
            list_dir,
            entry.path().to_str().unwrap(),
            entry.path().file_name().unwrap().to_str().unwrap(),
            );
            println!("{}", entry.path().to_str().unwrap());
        }
    }
    format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"> <title>{}</title></head><body><p>list:<br>{}</p></body></html>", dirname.file_name().unwrap().to_str().unwrap(), list_dir)
}