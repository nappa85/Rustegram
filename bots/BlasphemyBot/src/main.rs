extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::Telegram;

use std::env;
use std::fs::File;
use std::io::Read;

use serde_json::value::Value;

struct BlaspemyBot<'a> {
    token: &'a str,
    config: toml::Value,
}

impl<'a> BlaspemyBot<'a> {
    pub fn new(security_token: &str, cfg: toml::Value) -> BlaspemyBot {
        BlaspemyBot {
            token: security_token,
            config: cfg,
        }
    }

    pub fn parse(&self, json: Value) {
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 3, format!("Usage: {} <security_token> <json_request>", args.get(0).expect("Cannot find executable name")));

    let config_file = format!("{}.toml", args.get(0).expect("Cannot find executable name"));
    let mut toml = File::open(&config_file).expect(&format!("File {} not found", config_file));
    let mut s = String::new();
    toml.read_to_string(&mut s).expect("Unable to read Toml file");

    let bot = BlaspemyBot::new(args.get(1).expect("Cannot retrieve security token"), toml::from_str(&s).expect("Syntax error on Tolm file"));
    bot.parse(serde_json::from_str(args.get(2).expect("Cannot retrieve json request")).expect("Syntax error on json request"));
}
