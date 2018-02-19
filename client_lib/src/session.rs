extern crate serde_json;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde_json::value::Value;

//singleton
lazy_static! {
    pub static ref SESSION: Arc<Mutex<Session>> = Arc::new(Mutex::new(Session::new()));
}

pub struct Session {
    vars: HashMap<String, Value>,
}

impl Session {
    fn new() -> Session {
        Session {
            vars: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.vars.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<&Value> {
        self.vars.get(&key)
    }
}
