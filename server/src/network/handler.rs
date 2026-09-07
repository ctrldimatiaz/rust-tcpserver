use log::info;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct ConnectionHandler {}

impl ConnectionHandler {
    pub async fn handle(mut socket: TcpStream) -> Result<(), ()> {
        let mut buf = [0; 30];

        let _read_result = socket.read(&mut buf).await.unwrap();

        info!("Read result {}", String::from_utf8(buf.to_vec()).unwrap());

        let _write_result = socket.write(&buf).await.unwrap();
        Ok(())
    }
}
