use std::io::{Read, Write};
use std::net::TcpListener;
use std::error::Error;
mod http_parser;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buf: [u8; 1024] = [0; 1024];
                _stream.read(&mut buf).unwrap();
                let s = String::from_utf8_lossy(&buf);
                let mut req = http_parser::HttpRequest::new();
                let result = req.parse_request(&s);
                match result {
                    Ok(()) => {
                        _stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
                    }
                    Err(e) => {
                        _stream.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()).unwrap();
                    }
                }


            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}
