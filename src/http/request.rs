
#[allow(dead_code)]

pub struct HttpHeaders {
    host: String,
    user_agent: String,
    accept: String,
}

#[allow(unused)]
pub struct HttpRequest {
    method: String,
    target: String,
    version: String,
    headers: HttpHeaders,
    body: String,
}

impl HttpRequest {
    pub fn new(request_string: &str) -> HttpRequest {
        
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

        let headers = HttpHeaders { host, user_agent, accept };

        HttpRequest { method, target, version, headers, body }
    }

    pub fn response(&self) -> String {
        match self.target.as_str() {
            "/" => HttpRequest::response_ok(),
            t if t.starts_with("/echo") => HttpRequest::handle_echo(&self),
            t if t.starts_with("/user-agent") => HttpRequest::handle_user_agent(&self),
            _ => HttpRequest::response_not_found(),
        }
    }

    fn response_ok() -> String {
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string()
    }

    fn response_not_found() -> String {
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string()
    }

    // Handle Targets

    fn handle_echo(&self) -> String {
        let parts: Vec<&str> = self.target.split('/').collect();
        if parts.len() == 3 {
            return format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", parts[2].len(), parts[2]);
        }
        return "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string();
    }

    fn handle_user_agent(&self) -> String {
        format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", self.headers.user_agent.len(), self.headers.user_agent)
    }

}


