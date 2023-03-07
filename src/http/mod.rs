use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub struct HttpRequest {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpRequest {
    fn new() -> HttpRequest {
        HttpRequest {
            method: String::new(),
            path: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
}

impl From<String> for HttpRequest {
    fn from(s: String) -> Self {
        let mut lines = s.lines();
        let mut request = HttpRequest::new();

        if let Some(line) = lines.next() {
            let mut parts = line.split_whitespace();
            request.method = parts.next().unwrap().to_string();
            request.path = parts.next().unwrap().to_string();
        }

        for line in lines.clone() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();

            request.headers.insert(key, value);
        }

        request.body = lines.collect::<Vec<&str>>().join("\r\n");

        request
    }
}

impl From<TcpStream> for HttpRequest {
    fn from(stream: TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let mut request = HttpRequest::new();
        let mut buf_lines = buf_reader.lines();

        if let Some(line) = buf_lines.next() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();
            request.method = parts.next().unwrap().to_string();
            request.path = parts.next().unwrap().to_string();
        }

        for line in buf_lines {
            let line = line.unwrap();

            if line.is_empty() {
                break;
            }

            let mut parts = line.split(": ");
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();

            request.headers.insert(key, value);
        }

        request.body = buf_lines
            .collect::<Result<Vec<String>, _>>()
            .unwrap()
            .join("\r\n");

        request
    }
}

impl Debug for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpRequest {{ method: {}, path: {}, headers: {:?}, body: {} }}",
            self.method, self.path, self.headers, self.body
        )
    }
}
