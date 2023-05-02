use std::{collections::HashMap};
use std::option::Option;

pub struct Env {
	env: HashMap<String, String>,
}

impl Env {
	pub fn new () -> Self {
		let mut tmp_env:HashMap<String, String> = HashMap::new();
		for (key, value) in std::env::vars() {
			tmp_env.insert(key, value);
		}
		Env {
			env: tmp_env,
		}
	}
	pub fn get_env(&self, key: String) -> Option<&String> {
		self.env.get(&key)
	}
	pub fn set_env(&mut self, key: String, value: String) {
		self.env.insert(key, value);
	}
	pub fn remove_env(&mut self, key: String) {
		self.env.remove(&key);
	}
	pub fn print_env(&self) {
		for (key, value) in self.env.iter() {
			println!("{}={}", key, value);
		}
	}
}