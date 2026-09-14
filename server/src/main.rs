use std::sync::Arc;

use dotenv::dotenv;
use env_logger::Env;
use tokio::sync::Mutex;

use crate::{network::listener::KvListener, storage::store::Store};

pub mod network;
pub mod storage;

#[tokio::main]
async fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    let store = Arc::new(Mutex::new(Store::new()));

    let listener = KvListener::new(store);

    listener.boot().await;
}
