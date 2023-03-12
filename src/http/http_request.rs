use super::{HttpMethod, HttpVersion};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub query: HashMap<String, String>,
    pub version: HttpVersion,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpRequest {
    fn new() -> HttpRequest {
        HttpRequest {
            method: HttpMethod::UNINITIALIZED,
            path: String::new(),
            query: HashMap::new(),
            version: HttpVersion::UNINITIALIZED,
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
            request.method = HttpMethod::new(parts.next().unwrap());
            let path_with_query = parts.next().unwrap();
            request.path = path_with_query.split('?').next().unwrap().to_string();
            request.query = parts
                .next()
                .unwrap()
                .split('&')
                .fold(HashMap::new(), |mut acc, x| {
                    let mut parts = x.split('=');
                    let key = parts.next().unwrap().to_string();
                    let value = parts.next().unwrap().to_string();
                    acc.insert(key, value);
                    acc
                });
            request.version = HttpVersion::new(parts.next().unwrap());
        }

        for line in &mut lines {
            if line.is_empty() {
                break;
            }

            let mut parts = line.split(":");
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim().to_string();

            request.headers.insert(key, value);
        }

        request.body = lines.collect::<Vec<&str>>().join("\r\n");
        request
    }
}

impl Debug for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method.to_string())
            .field("path", &self.path)
            .field("query", &self.query)
            .field("version", &self.version.to_string())
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl Display for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HttpRequest {{ method: {}, path: {}, query: {:#?} version: {}, headers: {:#?}, body: {} }}",
            self.method.to_string(),
            self.path,
            self.query,
            self.version.to_string(),
            self.headers,
            self.body
        )
    }
}
