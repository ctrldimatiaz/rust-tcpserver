use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        println!("Accepting incoming connections at localhost:6379");

        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buf = [0; 30];

        let _read_result = socket.read(&mut buf).await.unwrap();

        println!("Read result {}", String::from_utf8(buf.to_vec()).unwrap());

        let _write_result = socket.write(&buf).await.unwrap();
    }
}
