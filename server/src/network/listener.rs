use std::{env, sync::Arc};

use log::{error, info};
use tokio::net::TcpListener;

use crate::{network::handler::ConnectionHandler, storage::store::Store};
use tokio::sync::Mutex;

pub struct KvListener {
    store: Arc<Mutex<Store>>,
}

impl KvListener {
    // Creates an instance of the struct responsible for listening and handling the received
    // messages
    pub fn new(store: Arc<Mutex<Store>>) -> Self {
        Self { store }
    }

    // Startup of key value server. Start listening for incoming messages and handle each one.
    pub async fn boot(&self) {
        let ip = env::var("IP").unwrap();

        let port: u16 = env::var("PORT")
            .unwrap()
            .parse()
            .expect("PORT must be a valid number");

        let listener = TcpListener::bind(format!("{}:{}", ip, port)).await.unwrap();

        loop {
            info!("Accepting incoming connections at {}:{}", ip, port);

            let (socket, _) = listener.accept().await.unwrap();

            let handler = ConnectionHandler::new(Arc::clone(&self.store));

            tokio::spawn(async move {
                if let Err(_e) = handler.handle(socket).await {
                    error!("Error handling connection");
                }
            });
        }
    }
}
