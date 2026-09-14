use std::collections::HashMap;

use kv_protocol::{Command, Response};

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    // New store struct responsible for storing the key value pairs
    pub fn new() -> Store {
        Self {
            data: HashMap::new(),
        }
    }

    // Get the value of the key parameter
    fn get(&self, key: &str) -> Response {
        match self.data.get(key) {
            Some(v) => Response::Value(v.clone()),
            None => Response::NotFound,
        }
    }

    // Set the value parameter on the key parameter
    fn set(&mut self, key: &str, value: &str) -> Response {
        self.data.insert(key.to_string(), value.to_string());
        Response::Ok
    }

    // Delete the stored key parameter
    fn delete(&mut self, key: &str) -> Response {
        self.data.remove(key);
        Response::Ok
    }

    // Facade for processing each command
    pub fn execute(&mut self, command: Command) -> Response {
        match command {
            Command::Get(key) => Store::get(self, &key),
            Command::Delete(key) => Store::delete(self, &key),
            Command::Set(key, value) => Store::set(self, &key, &value),
        }
    }
}
