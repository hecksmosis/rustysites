use super::{ContentType, HttpMethod, HttpRequest, HttpResponse, HttpVersion};
use std::{env, fs};

pub struct HttpRequestHandler {
    pub request: HttpRequest,
}

impl HttpRequestHandler {
    pub fn new(request: HttpRequest) -> HttpRequestHandler {
        HttpRequestHandler { request }
    }

    pub fn handle(&self) -> HttpResponse {
        let method = &self.request.method;

        match method {
            HttpMethod::GET => self.handle_get(),
            HttpMethod::POST => panic!("Method not implemented!"),
            HttpMethod::PUT => panic!("Method not implemented!"),
            HttpMethod::DELETE => panic!("Method not implemented!"),
            HttpMethod::HEAD => panic!("Method not implemented!"),
            HttpMethod::CONNECT => panic!("Method not implemented!"),
            HttpMethod::OPTIONS => panic!("Method not implemented!"),
            HttpMethod::TRACE => panic!("Method not implemented!"),
            HttpMethod::PATCH => panic!("Method not implemented!"),
            HttpMethod::UNINITIALIZED => panic!("Method not initialized!"),
        }
    }

    fn get_response_file_content(file: String) -> Option<ContentType> {
        let mut file_type = None;

        if let Some(dot) = file.find(".") {
            file_type = Some(file[dot + 1..].to_string());
        }

        match file_type {
            Some(file_type) => {
                if file_type == "html" {
                    return Some(ContentType::HTML);
                } else if file_type == "css" {
                    return Some(ContentType::CSS);
                } else if file_type == "js" {
                    return Some(ContentType::JS);
                } else {
                    return None;
                }
            }
            None => None,
        }
    }

    pub fn serve_file(&self, file: &str) -> HttpResponse {
        // Serves a file path without using the request
        let mut response = HttpResponse::default();
        let cwd = env::current_dir().unwrap();

        // Set the correct version
        response.version = HttpVersion::HTTP20;

        // Get the path to the file
        let mut path = cwd.join(file);

        // If the path is a directory, serve the index.html file
        if path.is_dir() {
            path = path.join("index.html");
        }

        // Get the content type of the file
        if let Some(content_type) =
            HttpRequestHandler::get_response_file_content(path.to_str().unwrap().to_string())
        {
            response
                .headers
                .insert("Content-Type".to_string(), content_type.to_string());
        } else {
            return HttpResponse::from(404);
        }

        // Read the file
        if let Ok(contents) = fs::read_to_string(path) {
            response.body = Some(contents);
        } else {
            return HttpResponse::from(404);
        }

        response
    }

    fn handle_get(&self) -> HttpResponse {
        let mut response = HttpResponse::default();
        let version = self.request.version.clone();

        response.version = version;

        let cwd = env::current_dir().unwrap();

        // Avoid path injection
        if let Some(back) = self.request.path.find("..") {
            if back > 0 {
                return HttpResponse::from(403);
            }
        }

        // Serve every directory inside of public/
        let mut path = cwd
            .join("public")
            .join(&self.request.path.strip_prefix("/").unwrap());
        if path.is_dir() {
            path = path.join("index.html");
        }

        if let Some(content_type) =
            HttpRequestHandler::get_response_file_content(path.to_str().unwrap().to_string())
        {
            response
                .headers
                .insert("Content-Type".to_string(), content_type.to_string());
        } else {
            return HttpResponse::from(404);
        }

        if let Ok(contents) = fs::read_to_string(path) {
            response.body = Some(contents);
        } else {
            return HttpResponse::from(404);
        }

        response
    }
}
