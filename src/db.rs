use std::time::Instant;
use std::{
    collections::{BTreeSet, HashMap},
    time::Duration,
};

#[derive(Debug, Default)]
pub struct DbStore {
    store: HashMap<String, String>,
    expire: BTreeSet<(Instant, String)>,
}

impl DbStore {
    pub fn new() -> Self {
        DbStore {
            store: HashMap::new(),
            expire: BTreeSet::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str, expire_time: Option<Duration>) {
        self.store
            .entry(key.to_string())
            .or_insert(value.to_string());

        if let Some(expire_time) = expire_time {
            self.expire
                .insert((Instant::now() + expire_time, key.to_string()));
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if let Some((time, key)) = self.expire.iter().find(|(_, in_key)| key == in_key) {
            if *time < Instant::now() {
                self.store.remove(key);
                self.expire.remove(&(*time, key.clone()));
                return None;
            }
        }
        self.store.get(key)
    }
}
