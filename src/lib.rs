
use std::net::TcpListener;
use std::io::Write;
use std::io::Read;

pub mod http;

/// Run the HTTP server
pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                
                println!("accepted new connection");
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();

                // Handle Request
                let http_request = http::request::HttpRequest::new(&String::from_utf8_lossy(&buffer[..]));
                let response = http_request.response();

                // Send Response Back
                stream.write_all(response.as_bytes()).unwrap();
                stream.flush().unwrap();
                
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}