use std::collections::HashMap;
use tokio::fs::File;
use tokio::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

// #[tokio::main]
// async fn main() {
//     let ipaddr = "192.168.31.174:8888";
//     let listener = TcpListener::bind(ipaddr).await.unwrap();
//     println!("{ipaddr}");

//     loop {
//         let (mut socket, addr) = listener.accept().await.unwrap();

//         let request = &mut [0; 1024];
//         let size = socket.read(request).await.unwrap();

//         let mut headers = [httparse::EMPTY_HEADER; 16];
//         let mut req = httparse::Request::new(&mut headers);
//         let res = req.parse(&request[..size]).unwrap();
//         if res.is_complete() {
//             if let Some(path) = req.path {
//                 match path {
//                     "/" | "/index.html" => {
//                         let body = tokio::fs::read_to_string("./index.html").await.unwrap();
//                         socket
//                         .write(
//                             format!(
//                                 "HTTP/1.1 200 OK\r\ncontent-type: text/html\r\ncontent-length: {}\r\n\r\n",
//                                 body.len(),
//                             )
//                             .as_bytes(),
//                         )
//                         .await
//                         .unwrap();
//                         socket.write(body.as_bytes()).await.unwrap();
//                     }
//                     "/test2.mp4" => {
//                         for header in req.headers {
//                             if header.name == "Range" {
//                                 let range;
//                                 range = String::from_utf8(header.value[6..].to_vec()).unwrap();
//                                 println!("{addr:?} - {range}");
//                                 let mut sp = range.split('-');
//                                 let begin = sp.next().unwrap().parse::<u64>().unwrap();
//                                 let mut end = sp.next().unwrap().parse::<u64>().unwrap_or_default();
//                                 if end == 0 {
//                                     end = begin + 1024 * 5;
//                                 }

//                                 let mut file = File::open("./test2.mp4").await.unwrap();
//                                 let file_len = file.metadata().await.unwrap().len();

//                                 file.seek(SeekFrom::Start(begin)).await.unwrap();

//                                 let buf = &mut [0; 64689];

//                                 let size = file.read(buf).await.unwrap();
//                                 if size < 64689 {
//                                     end = begin + size as u64;
//                                 }
//                                 socket
//                                     .write(
//                                         format!("HTTP/1.1 206 Partial Content\r\nAccept-Ranges: bytes\r\nContent-Length: {}\r\nContent-Range: bytes {}-{}/{}\r\nContent-Type: application/octet-stream\r\n\r\n", size, begin, end, file_len)
//                                             .as_bytes(),
//                                     )
//                                     .await
//                                     .unwrap();

//                                 let _ = socket.write(buf).await;
//                                 break;
//                             }
//                         }
//                     }
//                     _ => println!("{}", path),
//                 }
//             }
//         }
//     }
// }

enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    /// Request methods not standardized by the IETF
    NonStandard(String),
}

pub struct HttpVersion(pub u8, pub u8);

impl std::fmt::Display for HttpVersion {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}.{}", self.0, self.1)
    }
}

struct Request {
    method: Method,
    version: HttpVersion,
    path: String,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Request {
    pub async fn parse(read: &mut TcpStream) -> Self {
        loop {
            let buf = &mut [0; 1024];
        }
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let ipaddr = "127.0.0.1:8888";
    let listener = TcpListener::bind(ipaddr).await.unwrap();
    println!("{ipaddr}");
    let (mut sock, _) = listener.accept().await.unwrap();
    Request::parse(&mut sock).await;
}
