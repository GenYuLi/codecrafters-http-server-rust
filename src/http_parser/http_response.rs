use crate::http_parser::http_request::HttpRequest;
use std::error::Error;
use std::fmt::Display;



#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status_code: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HTTP/1.1 {}\r\n", self.status_code);
        for (key, value) in &self.headers {
            write!(f, "{}: {}\r\n", key, value);
        }
        write!(f, "\r\n{}", self.body)
    }
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            status_code: String::new(),
            headers: Vec::new(),
            body: String::new(),
        }
    }
}