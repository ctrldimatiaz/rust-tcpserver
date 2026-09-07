use kv_protocol::parse_command;
use log::{error, info};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct ConnectionHandler {}

impl ConnectionHandler {
    pub async fn handle(mut socket: TcpStream) -> Result<(), ()> {
        let mut buf_reader = BufReader::new(&mut socket);

        let mut message = String::new();
        let _read_result = buf_reader.read_line(&mut message).await.unwrap();

        let command = parse_command(message.as_str()).inspect_err(|e| {
            error!("Got error parsing command: {}", e);
        });

        if command.is_err() {
            socket.write_all(message.as_bytes()).await.unwrap();
            return Ok(());
        }

        info!("Read result {}", message);
        //let response = store.execute(command);

        socket.write_all(message.as_bytes()).await.unwrap();

        Ok(())
    }
}
