use std::collections::HashMap;

pub struct Store {
    data: HashMap<String, String>,
}

impl Store {
    pub fn new(data: HashMap<String, String>) -> Store {
        Self { data }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
}
