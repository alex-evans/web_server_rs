
use std::sync::Arc;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::Path;

use crate::Config;

#[allow(dead_code)]

pub struct HttpHeaders {
    host: String,
    user_agent: String,
    accept: String,
    accept_encoding: String,
}

#[allow(unused)]
pub struct HttpRequest {
    method: String,
    target: String,
    version: String,
    headers: HttpHeaders,
    content_length: usize,
    content_type: String,
    content: String,
    directory: String,
}

impl HttpRequest {
    pub fn new(request_string: &str, config: Arc<Config>) -> HttpRequest {

        // parse the full request string
        let lines: Vec<&str> = request_string.split("\r\n").collect();
        let request_line = lines.get(0).unwrap_or(&"");
        let headers_string = lines.iter().skip(1).take_while(|&&line| !line.is_empty()).map(|&s| s).collect::<Vec<&str>>().join("\r\n");
        let body = lines.iter().skip(2).map(|&s| s).collect::<Vec<&str>>().join("\r\n");

        // parse request line
        let parts: Vec<&str> = request_line.split_whitespace().collect();
        let method = parts.get(0).unwrap_or(&"").to_string();
        let target = parts.get(1).unwrap_or(&"").to_string();
        let version = parts.get(2).unwrap_or(&"").to_string();
        
        // parse headers
        let headers_parts: Vec<&str> = headers_string.split("\r\n").collect();
        let host = headers_parts.iter()
            .find(|&line| line.starts_with("Host: "))
            .map(|line| line.trim_start_matches("Host: ").to_string())
            .unwrap_or_default();
        let user_agent = headers_parts.iter()
            .find(|&line| line.starts_with("User-Agent: "))
            .map(|line| line.trim_start_matches("User-Agent: ").to_string())
            .unwrap_or_default();
        let accept = headers_parts.iter()
            .find(|&line| line.starts_with("Accept: "))
            .map(|line| line.trim_start_matches("Accept: ").to_string())
            .unwrap_or_default();
        let accept_encoding = headers_parts.iter()
            .find(|&line| line.starts_with("Accept-Encoding: "))
            .map(|line| line.trim_start_matches("Accept-Encoding: ").to_string())
            .unwrap_or_default();
        let headers = HttpHeaders { host, user_agent, accept, accept_encoding };

        // parse body
        let body_parts: Vec<&str> = body.split("\r\n").collect();
        let mut content_length: usize = 0;
        let mut content_type: String = "".to_string();
        let mut content: String = "".to_string();

        for line in &body_parts {
            if line.starts_with("Content-Length: ") {
                content_length = line.trim_start_matches("Content-Length: ").parse().unwrap_or(0);
                continue;
            };

            if line.starts_with("Content-Type: ") {
                content_type = line.trim_start_matches("Content-Type: ").to_string();
                continue;
            };
            
            content = line.chars().take(content_length).collect::<String>();
        }

        let directory = config.directory.clone();

        HttpRequest { method, target, version, headers, content_type, content_length, content, directory }
    }

    pub fn response(&self) -> String {
        match self.target.as_str() {
            "/" => self.response_ok(),
            t if t.starts_with("/echo") => self.handle_echo(),
            t if t.starts_with("/file") => self.handle_file(),
            t if t.starts_with("/user-agent") => self.handle_user_agent(),
            _ => self.response_not_found(),
        }
    }

    // Common Responses

    fn response_ok(&self) -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string()
    }

    fn response_not_found(&self) -> String {
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string()
    }

    // Handle Targets

    fn handle_echo(&self) -> String {
        let parts: Vec<&str> = self.target.split('/').collect();
        
        if parts.len() != 3 {
            return self.response_not_found()
        }
        
        if self.headers.accept_encoding.contains("gzip") {
            return format!("HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", parts[2].len(), parts[2]);
        }
        
        return format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", parts[2].len(), parts[2]);
    }

    fn handle_file(&self) -> String {
        // HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 14\r\n\r\nHello, World!
        let parts: Vec<&str> = self.target.split('/').collect();
        
        if parts.len() != 3 {
            return self.response_not_found();
        }

        if self.method == "GET" {
            return self.get_file(parts[2]);
        } else if self.method == "POST" {
            return self.post_file(parts[2]);
        } else {
            return self.response_not_found();
        }

    }

    fn get_file(&self, file_name: &str) -> String {
        let file_path = Path::new(&self.directory).join(file_name);
        if file_path.exists() && file_path.is_file() {
            let mut file = File::open(file_path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            return format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
        };
        return self.response_not_found();
    }

    fn post_file(&self, file_name: &str) -> String {
        let file_path = Path::new(&self.directory).join(file_name);
        let mut file = File::create(file_path).unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
        return format!(
            "HTTP/1.1 201 Created\r\n\r\n"
        );
    }

    fn handle_user_agent(&self) -> String {
        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", self.headers.user_agent.len(), self.headers.user_agent)
    }

}


