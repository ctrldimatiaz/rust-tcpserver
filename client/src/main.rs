use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() {
    let mut connector = TcpStream::connect("127.0.0.1:6379").await.unwrap();

    let message = String::from("hello world");

    let _result = connector.write(message.as_bytes()).await.unwrap();

    let mut buf = [0; 30];

    let _read_result = connector.read(&mut buf).await.unwrap();

    println!(
        "Sent {} and got {}",
        message,
        String::from_utf8(buf.to_vec()).unwrap(),
    );
}
