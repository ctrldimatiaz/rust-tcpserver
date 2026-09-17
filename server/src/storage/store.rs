use std::{collections::HashMap, env, path::Path};

use kv_protocol::{Command, Response};
use tokio::fs;

pub struct Store {
    data: HashMap<String, String>,
    file_path: String,
}

impl Store {
    // New store struct responsible for storing the key value pairs
    pub async fn new() -> Store {
        let file_path = env::var("STORAGE").unwrap();

        if !Path::new(&file_path).try_exists().unwrap_or(false) {
            fs::write(&file_path, "{}").await.unwrap();
        }

        let contents = fs::read_to_string(&file_path)
            .await
            .unwrap_or(String::from("{}"));

        let data: HashMap<String, String> = serde_json::from_str(&contents).unwrap();

        Self { data, file_path }
    }

    // Get the value of the key parameter
    fn get(&self, key: &str) -> Response {
        match self.data.get(key) {
            Some(v) => Response::Value(v.clone()),
            None => Response::NotFound,
        }
    }

    // Set the value parameter on the key parameter
    async fn set(&mut self, key: &str, value: &str) -> Response {
        self.data.insert(key.to_string(), value.to_string());
        Store::update_storage(self).await;
        Response::Ok
    }

    // Delete the stored key parameter
    async fn delete(&mut self, key: &str) -> Response {
        self.data.remove(key);
        Store::update_storage(self).await;
        Response::Ok
    }

    async fn update_storage(&self) {
        fs::write(&self.file_path, serde_json::to_string(&self.data).unwrap())
            .await
            .unwrap();
    }

    // Facade for processing each command
    pub async fn execute(&mut self, command: Command) -> Response {
        match command {
            Command::Get(key) => Store::get(self, &key),
            Command::Delete(key) => Store::delete(self, &key).await,
            Command::Set(key, value) => Store::set(self, &key, &value).await,
        }
    }
}
