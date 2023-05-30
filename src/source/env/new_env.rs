use std::collections::HashMap;
use std::option::Option;

pub struct Env {
    env: HashMap<String, String>,
}

impl Env {
    pub fn new() -> Self {
        let mut tmp_env: HashMap<String, String> = HashMap::new();
        for (key, value) in std::env::vars() {
            tmp_env.insert(key, value);
        }
        Env { env: tmp_env }
    }
    pub fn get_env(&self, key: &str) -> Option<&String> {
        self.env.get(key)
    }
    pub fn set_env(&mut self, key: String, value: String) {
        self.env.insert(key, value);
    }
    pub fn remove_env(&mut self, key: String) {
        self.env.remove(&key);
    }
    pub fn get_all(&self) -> HashMap<String, String> {
        self.env.clone()
    }
    pub fn path_validation(&self, cmd: &str) -> bool {
        let splitted: Vec<&str> = self.get_env("PATH").unwrap().split(':').collect();
        for path in splitted {
            let mut tmp_path = path.to_string();
            tmp_path.push('/');
            tmp_path.push_str(cmd);
            if std::path::Path::new(&tmp_path).exists() {
                return true;
            }
        }
        false
    }
}
