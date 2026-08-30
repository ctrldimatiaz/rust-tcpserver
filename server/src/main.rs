use dotenv::dotenv;
use env_logger::Env;
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        info!("Accepting incoming connections at localhost:6379");

        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buf = [0; 30];

        let _read_result = socket.read(&mut buf).await.unwrap();

        info!("Read result {}", String::from_utf8(buf.to_vec()).unwrap());

        let _write_result = socket.write(&buf).await.unwrap();
    }
}
