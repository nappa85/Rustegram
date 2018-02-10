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

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3, format!("Usage: {} <security_token> <json_request>", args.get(0).expect("Cannot find executable name")));

    let path = Path::new(args.get(0).expect("Cannot find executable path"));
    let config_file = format!("{}.toml", path.file_stem().expect("Cannot find executable name").to_str().expect("Cannot parse executable name"));
    let mut toml = File::open(&config_file).expect(&format!("File {} not found", config_file));
    let mut s = String::new();
    toml.read_to_string(&mut s).expect("Unable to read Toml file");

    let bot = Telegram::init_bot(BlaspemyBot::new, args.get(1).expect("Cannot retrieve security token"), toml::from_str(&s).expect("Syntax error on Tolm file"));
    bot.parse(serde_json::from_str(args.get(2).expect("Cannot retrieve json request")).expect("Syntax error on json request")).expect("Error parsing request");
}
