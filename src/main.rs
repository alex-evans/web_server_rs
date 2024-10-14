use codecrafters_http_server::run;

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");
    run().await.unwrap();
}

