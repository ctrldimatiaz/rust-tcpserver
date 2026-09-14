use std::env;

use dotenv::dotenv;
use env_logger::Env;
use log::info;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    let ip = env::var("IP").unwrap();

    let port: u16 = env::var("PORT")
        .unwrap()
        .parse()
        .expect("PORT must be a valid number");

    let mut connector = TcpStream::connect(format!("{}:{}", ip, port))
        .await
        .unwrap();

    let message = String::from("GET test");

    let _result = connector
        .write(format!("{message}\n").as_bytes())
        .await
        .unwrap();

    let mut reader = BufReader::new(&mut connector);
    let mut line = String::new();

    // Read a single line
    reader.read_line(&mut line).await.unwrap();
    println!("Received: {}", line.trim());

    info!("Sent {} and got {}", message, line);
}
