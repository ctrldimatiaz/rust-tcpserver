use dotenv::dotenv;
use env_logger::Env;

use crate::network::listener::KvListener;

pub mod network;
pub mod storage;

#[tokio::main]
async fn main() {
    // load .env
    dotenv().ok();

    let env = Env::default().filter_or("RUST_LOG", "info");

    env_logger::init_from_env(env);

    KvListener::boot().await;
}
