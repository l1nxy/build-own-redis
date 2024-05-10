use std::collections::HashMap;
#[derive(Debug, Default)]
pub struct DbStore {
    store: HashMap<String, String>,
}

impl DbStore {
    pub fn new() -> Self {
        DbStore {
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.store
            .entry(key.to_string())
            .or_insert(value.to_string());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }
}
