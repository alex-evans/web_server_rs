#[allow(unused_imports)]
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
                println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
                let (request, _headers, _body) = parse_request(&String::from_utf8_lossy(&buffer[..]));
                let response = process_request_string(&request);

                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

/// Process the request and return the request line, headers and body
fn parse_request(request: &str) -> (String, String, String) {
    let lines: Vec<&str> = request.split("\r\n").collect();
    let first_line = lines.get(0).unwrap_or(&"");
    let second_line = lines.get(1).unwrap_or(&"");
    let remaining_lines = lines.iter().skip(2).map(|&s| s).collect::<Vec<&str>>().join("\r\n");
    let first_line = first_line.to_string();
    let second_line = second_line.to_string();
    let remaining_lines = remaining_lines.to_string();
    (first_line, second_line, remaining_lines)
}

/// Process the request and return the response
fn process_request_string(request: &str) -> String {
    let parts: Vec<&str> = request.split_whitespace().collect();
    let _method = parts.get(0).unwrap_or(&"");
    let target = parts.get(1).unwrap_or(&"");
    let _version = parts.get(2).unwrap_or(&"");

    if target.to_string() == "/" {
        return response_ok();
    } else {
        return response_not_found();
    }
}

/// Return a 200 OK response
fn response_ok() -> String {
    "HTTP/1.1 200 OK\r\n\r\n".to_string()
}

/// Return a 404 Not Found response
fn response_not_found() -> String {
    "HTTP/1.1 404 Not Found\r\n\r\n".to_string()
}
