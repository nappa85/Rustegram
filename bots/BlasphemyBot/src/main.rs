extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::{Bot, Telegram};

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::boxed::Box;

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

    fn parse(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }
}

impl BlaspemyBot {
    fn dispatch(&self, method: &str) -> Result<Value, String> {
        match method {
            "about" => self.about(),
            "help" => self.help(),
            "swear" => self.swear(),
            "swearto" => self.swearto(),
            "blackhumor" => self.blackhumor(),
            _ => Err(format!("Method {} not found", method)),
        }
    }

    fn about(&self) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn help(&self) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn swear(&self) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn swearto(&self) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn blackhumor(&self) -> Result<Value, String> {
        Err(String::from("TODO"))
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3, format!("Usage: {} <security_token> <json_request>", args.get(0).expect("Cannot find executable name")));

    let config_file = format!("{}.toml", args.get(0).expect("Cannot find executable name"));
    let mut toml = File::open(&config_file).expect(&format!("File {} not found", config_file));
    let mut s = String::new();
    toml.read_to_string(&mut s).expect("Unable to read Toml file");

    let bot = Telegram::init_bot(BlaspemyBot::new, args.get(1).expect("Cannot retrieve security token"), toml::from_str(&s).expect("Syntax error on Tolm file"));
    bot.parse(serde_json::from_str(args.get(2).expect("Cannot retrieve json request")).expect("Syntax error on json request")).expect("Error parsing request");
}
