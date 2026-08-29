use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() {
    let mut connector = TcpStream::connect("127.0.0.1:6379").await.unwrap();

    let buf = String::from("hello world");
    let result = connector.write(buf.as_bytes()).await.unwrap();
    println!("Sent {} and got {}", buf, result);
}
