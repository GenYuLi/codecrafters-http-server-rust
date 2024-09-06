use std::error::Error;
use std::sync::{Arc, RwLock};
use std::marker::{Sync, Send};

#[derive(Debug)]
pub(crate) struct HttpRequest {
    // Use pub for debugging purposes
    pub method: Arc<String>,
    pub request_uri: Arc<String>,
    pub http_version: Arc<String>,
    pub headers: RwLock<Vec<(String, String)>>,
    pub body: Arc<String>,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        HttpRequest {
            method: Arc::new(String::new()),
            request_uri: Arc::new(String::new()),
            http_version: Arc::new(String::new()),
            headers: RwLock::new(Vec::new()),
            body: Arc::new(String::new()),
        }
    }

    pub fn parse_request(&mut self, request: &str) -> Result<(), Box<dyn Error>> {
        let mut lines = request.lines();
        let request_line = lines.next().ok_or("no request line").unwrap();
        println!("request line: {}", request_line);
        let mut parts = request_line.split_whitespace();
        self.method = Arc::new(parts.next().ok_or("method error").unwrap().to_string());
        self.request_uri = Arc::new(parts.next().ok_or("url error").unwrap().to_string());
        self.http_version = Arc::new(parts.next().ok_or("version error{}").unwrap().to_string());
        loop {
            let line = lines.next().ok_or("header error").unwrap().to_string();
            if line == "" {
                break;
            }
            let mut header_parts = line.split(": ");
            let key = header_parts.next().ok_or("header key error").unwrap().to_string();
            let value = header_parts.next().ok_or("header value error").unwrap().to_string();
            self.headers.write().unwrap().push((key, value))
        }

        self.body = Arc::new(lines.next().ok_or("body error").unwrap().to_string());

        // TODO: Parse headers and body
        Ok(())
    }
}