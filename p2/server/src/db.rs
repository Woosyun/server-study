use std::{
    collections::HashMap,
    path::PathBuf,
    fs,
};
use serde::{Serialize, Deserialize};
use serde_json;

pub type Key = u32;
pub type Value = String;

#[derive(Serialize, Deserialize)]
pub struct Server {
    workdir: PathBuf,
    db: HashMap<Key, Value>,
    expired: HashMap<Key, u64>,
}
impl Server {
    fn new(workdir: PathBuf) -> Self {
        Self {
            workdir,
            db: HashMap::new(),
            expired: HashMap::new(),
        }
    }
    fn file_name() -> &'static str {
        "server_info.json"
    }
    pub fn build(workdir: PathBuf) -> Result<Self, String> {
        let mut path = workdir;
        path.push(Server::file_name());
        if !path.exists() {
            Ok(Server::new(path))
        } else {
            let content = fs::read_to_string(path)
                .map_err(|_| format!("cannot read file"))?;
            let server = serde_json::from_str(&content)
                .map_err(|_| format!("cannot parse file"))?;
            Ok(server)
        }
    }

    pub fn get(&mut self, key: &Key) -> Option<Value> {
        if let Some(expire_time) = self.expired.get(key) {
            if *expire_time < Server::current_time() {
                let _ = self.db.remove(key);
                let _ = self.expired.remove(key);
                println!("{:?}", self.store());
                return None;
            }
        }

        self.db.get(key).cloned()
    }
    pub fn set(&mut self, key: Key, value: Value) {
        self.db.insert(key, value);
        println!("{:?}", self.store());
    }
    pub fn expire(&mut self, key: Key, duration: u32) {
        let expire_time = Server::current_time() + duration as u64;
        self.expired.insert(key, expire_time);
        println!("{:?}", self.store());
    }

    pub fn current_time() -> u64 {
        use std::time::SystemTime;
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_secs();
        now
    }

    pub fn store(&self) -> Result<(), String> {
        let mut lock_file = self.workdir.clone();
        lock_file.set_extension("lock");

        println!("lock file: {:?}", &lock_file);

        if !lock_file.exists() {
            println!("can create lock file");
            let content = serde_json::to_string(self)
                .map_err(|_| format!("cannot stringify"))?;
            fs::write(&lock_file, content)
                .map_err(|_| format!("cannot write"))?;
            fs::rename(&lock_file, &self.workdir)
                .map_err(|_| format!("cannot rename"))?;
        }
        Ok(())
    }
}
