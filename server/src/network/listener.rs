use log::{error, info};
use tokio::net::TcpListener;

use crate::network::handler::ConnectionHandler;

pub struct KvListener {}

impl KvListener {
    pub async fn boot() {
        let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

        loop {
            info!("Accepting incoming connections at localhost:6379");

            let (socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                if let Err(_e) = ConnectionHandler::handle(socket).await {
                    error!("Error handling connection");
                }
            });
        }
    }
}
