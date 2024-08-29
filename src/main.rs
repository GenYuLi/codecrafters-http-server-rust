use std::io::{Read, Write};
use std::net::TcpListener;
use std::error::Error;
pub mod http_parser;
pub mod router;
use http_parser::http_request::HttpRequest;
use http_parser::http_response::HttpResponse;
use router::Router;

fn main() {
    let mut my_router = Router::new();
    my_router
        .add_route("/", |_req| {
            HttpResponse{
                status_code: "200 OK".to_string(),
                headers: vec![],
                body: "".to_string(),
            }
        })
        .add_route("/echo/*", |req| {
            let mut body = req.request_uri.split("/").collect::<Vec<&str>>()[2];
            let content_length: i32 = body.len().try_into().unwrap();
            HttpResponse{
                status_code: "200 OK".to_string(),
                headers: vec![("Content-Type".to_string(), "text/plain".to_string()), ("Content-Length".to_string(), content_length.to_string())],
                body: body.to_string(),
            }
        });

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
                let mut req = HttpRequest::new();
                let result = req.parse_request(&s);
                match result {
                    Ok(()) => {
                        println!("request: {:?}", req);
                        let handler = my_router.find_route(&req.request_uri);
                        match handler {
                            Some(handler) => {
                                let response = handler(&req);
                                _stream.write(response.to_string().as_bytes()).unwrap();
                            }
                            None => {
                                _stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        _stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}
