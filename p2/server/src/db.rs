use serde::{Serialize, Deserialize};
use std::{
    fs,
    path::PathBuf,
    collections::HashMap,
};

pub type Key = u64;
pub type Time = u64;
pub type Value = String;

#[derive(Serialize, Deserialize)]
pub struct Server {
    workdir: PathBuf,
    db: HashMap<Key, Value>,
    expire: HashMap<Key, Time>
}
impl Server {
    pub fn build(workdir: PathBuf) -> Self {
        let path = workdir.join(Server::file_name());
        if path.exists() {
            let content = fs::read_to_string(path).unwrap();
            let server: Server = serde_json::from_str(&content).unwrap();
            return server;
        }

        Self {
            workdir,
            db: HashMap::new(),
            expire: HashMap::new(),
        }
    }
    fn file_name() -> &'static str {
        "server"
    }
    fn current_time(&self) -> Time {
        use std::time::SystemTime;
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs()
    }
    pub fn get(&mut self, key: Key) -> Option<Value> {
        // expired
        if let Some(time) = self.expire.get(&key) {
            if *time <= self.current_time() {
                println!("key {} is expired", key);
                self.db.remove(&key);
                return None;
            }
        }

        self.store();
        self.db.get(&key).cloned()
    }
    pub fn set(&mut self, key: Key, value: Value) {
        self.db.insert(key, value);
        self.store()
    }
    pub fn expires(&mut self, key: Key, duration: u64) {
        let time = self.current_time() + duration;
        self.expire.insert(key, time);
        println!("key {} will expires at {}", key, time);
        self.store()
    }
    fn store(&self) {
        let mut lock = self.workdir.join(Server::file_name());
        lock.set_extension("lock");
        if !lock.exists() {
            let content = serde_json::to_string(&self).unwrap();
            fs::write(&lock, &content).unwrap();
            let mut path = lock.clone();
            path.set_extension("");
            fs::rename(&lock, &path).unwrap();
        }
    }
}
