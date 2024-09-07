use std::sync::{Arc, RwLock};
use std::{env, fs, thread};
use std::path::Path;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::error::Error;
pub mod http_parser;
pub mod router;
use http_parser::http_request::HttpRequest;
use http_parser::http_response::HttpResponse;
use router::Router;

fn main() -> Result<(), Box<dyn Error + Send>> {
    let my_router = Arc::new(RwLock::new(Router::new()));
    {
        let mut router = my_router.write().unwrap();
        router
            .add_route("/", |_req| {
                Ok(HttpResponse{
                    status_code: "200 OK".to_string(),
                    headers: vec![],
                    body: "".to_string(),
                })
            })
            .add_route("/echo/*", |req| {
                let body = req.request_uri.split("/").collect::<Vec<&str>>()[2];
                let content_length: i32 = body.len().try_into()?;
                Ok(HttpResponse{
                    status_code: "200 OK".to_string(),
                    headers: vec![("Content-Type".to_string(), "text/plain".to_string()), ("Content-Length".to_string(), content_length.to_string())],
                    body: body.to_string(),
                })
            })
            .add_route("/user-agent", |req| {
                let headers = req.headers.read().unwrap();
                let user_agent_opt = headers.iter().find(|(key, _)| key.to_lowercase() == "user-agent");
                let user_agent = match user_agent_opt {
                    Some((_, value)) => value,
                    None => "unknown",
                }.to_string();
                let content_length: i32 = user_agent.len().try_into().unwrap();
                println!("user-agent: {}", user_agent);
                Ok(HttpResponse{
                    status_code: "200 OK".to_string(),
                    headers: vec![("Content-Type".to_string(), "text/plain".to_string()), ("Content-Length".to_string(), content_length.to_string())],
                    body: user_agent.to_string(),
                })
            })
            .add_route("/files/*", |_req| {
                let file_dir = Path::new("/tmp/data/codecrafters.io/http-server-tester/");
                let filename = _req.request_uri.split("/").collect::<Vec<&str>>()[2];
                let file_in_binary_dir = file_dir.join(filename);
                let content = fs::read_to_string(file_in_binary_dir);
                let content = match content {
                    Ok(content) => content,
                    Err(_) => return Ok(HttpResponse{
                        status_code: "404 Not Found".to_string(),
                        headers: vec![],
                        body: "".to_string(),
                    }),
                };
                let content_length: i32 = content.len().try_into().unwrap();
                Ok(HttpResponse{
                    status_code: "200 OK".to_string(),
                    headers: vec![("Content-Type".to_string(), "application/octet-stream".to_string()), ("Content-Length".to_string(), content_length.to_string())],
                    body: content,
                })
            });
    }

    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {

                let my_router = Arc::clone(&my_router);

                thread::spawn(move || -> Result<(), Box<dyn Error + Send>> {
                    let my_router = my_router.clone();
                    println!("accepted new connection");
                    let mut buf: [u8; 1024] = [0; 1024];
                    // Read the request into the buffer
                    _stream
                        .read(&mut buf)
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
                    let s = String::from_utf8_lossy(&buf);
                    println!("request: {}", s);
                    let mut req = HttpRequest::new();
                    let result = req.parse_request(&s);
                    match result {
                        Ok(()) => {
                            println!("request: {:?}", req);
                            let router = my_router.read().unwrap();
                            let handler = router.find_route(&req.request_uri);
                            match handler {
                                Some(handler) => {
                                    let response = handler(&req).unwrap();
                                    _stream.write(response.to_string().as_bytes()).map_err(|e| Box::new(e) as Box<dyn Error + Send>)?;
                                }
                                None => {
                                    _stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
                                }
                            }
                        }
                        Err(_e) => {
                            println!("error: {}", _e);
                            _stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes());
                        }
                    }
                    Ok(())
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())

}
