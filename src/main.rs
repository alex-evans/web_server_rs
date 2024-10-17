
use std::env;

use codecrafters_http_server::{
    Config,
    run,
};

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");
    let config = Config::from_args(env::args().collect()).unwrap();
    run(config).await.unwrap();
}

