use std::error::Error;

#[derive(Debug)]
pub(crate) struct HttpRequest {
    // Use pub for debugging purposes
    pub method: String,
    pub request_uri: String,
    pub http_version: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl HttpRequest {
    pub fn new() -> HttpRequest {
        HttpRequest {
            method: String::new(),
            request_uri: String::new(),
            http_version: String::new(),
            headers: Vec::new(),
            body: String::new(),
        }
    }

    pub fn parse_request(&mut self, request: &str) -> Result<(), Box<dyn Error>> {
        let mut lines = request.lines();
        let request_line = lines.next().ok_or("no request line")?;
        println!("{}", request_line);
        let mut parts = request_line.split_whitespace();
        self.method = parts.next().ok_or("method error")?.to_string();
        self.request_uri = parts.next().ok_or("url error")?.to_string();
        self.http_version = parts.next().ok_or("version error{}")?.to_string();

        // TODO: Parse headers and body
        Ok(())
    }
}