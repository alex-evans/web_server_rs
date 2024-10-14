
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub mod http;

/// Run the HTTP server
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:4221").await?;
    
    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream).await {
                        eprintln!("an error occurred; error = {:?}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("failed to accept client; error = {:?}", e);
            }
        }
    }

}

async fn handle_connection(stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let stream = Arc::new(Mutex::new(stream));
    let mut buf = [0; 1024];

    loop {
        let mut stream_lock = stream.lock().await;
        match stream_lock.read(&mut buf).await {
            Ok(0) => {
                println!("Connection closed");
                return Ok(());
            }
            Ok(n) => {
                
                let http_request = http::request::HttpRequest::new(std::str::from_utf8(&buf[..n])?);
                let response = http_request.response();
                
                stream_lock.write_all(response.as_bytes()).await?;
                stream_lock.flush().await?;
                
                return Ok(());
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                return Ok(());
            }
        }
    }
}