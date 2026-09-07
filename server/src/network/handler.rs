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

        let command = parse_command(message.as_str());

        if let Err(e) = command {
            let error_message = format!("Got error parsing command: {}", e);

            error!("{error_message}");

            let writeresult = socket.write_all(error_message.as_bytes()).await;

            if let Err(ew) = writeresult {
                error!("Got error writing back the result: {}", ew);
                return Err(());
            }
            return Err(());
        }

        info!("Read result {}", message);
        //let response = store.execute(command);

        socket.write_all(message.as_bytes()).await.unwrap();

        Ok(())
    }
}
