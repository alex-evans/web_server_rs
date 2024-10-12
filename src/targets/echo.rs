

pub fn handle_echo(target_path: &str) -> String {
    let parts: Vec<&str> = target_path.split('/').collect();
    if parts.len() == 3 {
        return format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", parts[2].len(), parts[2]);
    }
    return "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 4\r\n\r\ntest".to_string();
}