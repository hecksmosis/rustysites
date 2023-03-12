use crate::Template;

use super::HttpVersion;
use std::{collections::HashMap, env, fs};

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: String,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpResponse {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut response = String::new();

        response.push_str(&format!(
            "{} {} {}\r\n",
            self.version.to_string(),
            self.status_code,
            self.status_text
        ));

        if self.headers.len() > 0 {
            for (key, value) in self.headers.iter() {
                response.push_str(&format!("{}: {}\r\n", key, value));
            }
        } else {
            response.push_str("Content-Type: text/html\r\n");
        }

        response.push_str("\r\n");

        if let Some(body) = &self.body {
            response.push_str(body);
        }

        response.as_bytes().to_vec()
    }
}

impl Default for HttpResponse {
    fn default() -> HttpResponse {
        HttpResponse {
            version: HttpVersion::UNINITIALIZED,
            status_code: String::new(),
            status_text: String::new(),
            headers: HashMap::new(),
            body: None,
        }
    }
}

impl From<u16> for HttpResponse {
    fn from(status_code: u16) -> Self {
        let mut response = HttpResponse::default();
        response.version = HttpVersion::HTTP11;
        response.status_code = status_code.to_string();
        response.status_text = match status_code {
            403 => "Forbidden",
            404 => "Not Found",
            _ => "Unknown",
        }
        .to_string();

        let cwd = env::current_dir().unwrap();
        let path = cwd.join("private").join(format!("{}.html", status_code));

        if let Ok(body) = fs::read_to_string(path) {
            response.body = Some(body);
        }

        response
    }
}

impl From<String> for HttpResponse {
    fn from(text: String) -> Self {
        let mut response = HttpResponse::default();
        response.version = HttpVersion::HTTP11;
        response.status_code = "200".to_string();
        response.status_text = "OK".to_string();

        let cwd = env::current_dir().unwrap();
        let path = cwd.join("templates").join("text.html");

        if let Ok(body) = fs::read_to_string(path) {
            response.body = Some(body.replace("{{body}}", &text));
        }

        response
    }
}

impl From<Template> for HttpResponse {
    fn from(mut template: Template) -> Self {
        let mut response = HttpResponse::default();
        response.version = HttpVersion::HTTP20;
        response.status_code = "200".to_string();
        response.status_text = "OK".to_string();

        if let Some(body) = &template.rendered {
            response.body = Some(body.to_string());
        } else {
            template.render(HashMap::new());
        }
        response
    }
}
