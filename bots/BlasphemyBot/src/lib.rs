extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::{Bot, Telegram};

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde_json::value::Value;

struct BlaspemyBot {
    api: Telegram,
    config: toml::Value,
}

impl Bot for BlaspemyBot {
    fn new(api: Telegram, cfg: toml::Value) -> BlaspemyBot {
        BlaspemyBot {
            api: api,
            config: cfg,
        }
    }

    fn dispatch(&self, method: &str, json: Value) -> Result<Value, String> {
        match method {
            "about" => self.about(json),
            "help" => self.help(json),
            "swear" => self.swear(json),
            "swearto" => self.swearto(json),
            "blackhumor" => self.blackhumor(json),
            _ => Err(format!("Method {} not found", method)),
        }
    }
}

impl BlaspemyBot {
    fn about(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn help(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn swear(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn swearto(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn blackhumor(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
