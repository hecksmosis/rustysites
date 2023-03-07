pub use http_request::HttpRequest;
pub use http_request_handler::HttpRequestHandler;
pub use http_response::HttpResponse;

pub mod http_request;
pub mod http_request_handler;
pub mod http_response;

// Define important enums
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
    UNINITIALIZED,
}

impl HttpMethod {
    pub fn new(s: &str) -> HttpMethod {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "HEAD" => HttpMethod::HEAD,
            "CONNECT" => HttpMethod::CONNECT,
            "OPTIONS" => HttpMethod::OPTIONS,
            "TRACE" => HttpMethod::TRACE,
            "PATCH" => HttpMethod::PATCH,
            _ => HttpMethod::UNINITIALIZED,
        }
    }
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match self {
            HttpMethod::GET => "GET".to_string(),
            HttpMethod::POST => "POST".to_string(),
            HttpMethod::PUT => "PUT".to_string(),
            HttpMethod::DELETE => "DELETE".to_string(),
            HttpMethod::HEAD => "HEAD".to_string(),
            HttpMethod::CONNECT => "CONNECT".to_string(),
            HttpMethod::OPTIONS => "OPTIONS".to_string(),
            HttpMethod::TRACE => "TRACE".to_string(),
            HttpMethod::PATCH => "PATCH".to_string(),
            HttpMethod::UNINITIALIZED => "UNINITIALIZED".to_string(),
        }
    }
}

impl Clone for HttpMethod {
    fn clone(&self) -> Self {
        match self {
            HttpMethod::GET => HttpMethod::GET,
            HttpMethod::POST => HttpMethod::POST,
            HttpMethod::PUT => HttpMethod::PUT,
            HttpMethod::DELETE => HttpMethod::DELETE,
            HttpMethod::HEAD => HttpMethod::HEAD,
            HttpMethod::CONNECT => HttpMethod::CONNECT,
            HttpMethod::OPTIONS => HttpMethod::OPTIONS,
            HttpMethod::TRACE => HttpMethod::TRACE,
            HttpMethod::PATCH => HttpMethod::PATCH,
            HttpMethod::UNINITIALIZED => HttpMethod::UNINITIALIZED,
        }
    }
}

pub enum HttpVersion {
    HTTP10,
    HTTP11,
    HTTP20,
    UNINITIALIZED,
}

impl HttpVersion {
    pub fn new(s: &str) -> HttpVersion {
        match s.to_uppercase().as_str() {
            "HTTP/1.0" => HttpVersion::HTTP10,
            "HTTP/1.1" => HttpVersion::HTTP11,
            "HTTP/2.0" => HttpVersion::HTTP20,
            _ => HttpVersion::UNINITIALIZED,
        }
    }
}

impl Clone for HttpVersion {
    fn clone(&self) -> Self {
        match self {
            HttpVersion::HTTP10 => HttpVersion::HTTP10,
            HttpVersion::HTTP11 => HttpVersion::HTTP11,
            HttpVersion::HTTP20 => HttpVersion::HTTP20,
            HttpVersion::UNINITIALIZED => HttpVersion::UNINITIALIZED,
        }
    }
}

impl ToString for HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP10 => "HTTP/1.0".to_string(),
            HttpVersion::HTTP11 => "HTTP/1.1".to_string(),
            HttpVersion::HTTP20 => "HTTP/2.0".to_string(),
            HttpVersion::UNINITIALIZED => "UNINITIALIZED".to_string(),
        }
    }
}

pub enum ContentType {
    HTML,
    CSS,
    JS,
    UNINITIALIZED,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::HTML => "text/html".to_string(),
            ContentType::CSS => "text/css".to_string(),
            ContentType::JS => "text/javascript".to_string(),
            ContentType::UNINITIALIZED => "UNINITIALIZED".to_string(),
        }
    }
}
