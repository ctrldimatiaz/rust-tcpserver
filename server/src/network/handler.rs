use std::sync::Arc;

use kv_protocol::parse_command;
use log::{error, info};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::storage::store::Store;

pub struct ConnectionHandler {
    store: Arc<Mutex<Store>>,
}

impl ConnectionHandler {
    // New handler with shared Store reference
    pub fn new(store: Arc<Mutex<Store>>) -> Self {
        Self { store }
    }

    // Responsible for reading, parsing and processing the incoming command
    pub async fn handle(&self, mut socket: TcpStream) -> Result<(), ()> {
        // Read incoming message

        let mut buf_reader = BufReader::new(&mut socket);

        let mut message = String::new();
        let _read_result = buf_reader.read_line(&mut message).await.unwrap();

        // Parse the command and stop in case of error

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

        // Process the command and store it
        let mut store = self.store.lock().await;

        let response = store.execute(command.unwrap()).await;

        socket.write_all(response.as_bytes()).await.unwrap();

        Ok(())
    }
}
