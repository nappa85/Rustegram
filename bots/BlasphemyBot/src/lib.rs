extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::{Bot, Telegram};

use std::fs::File;
use std::io::Read;

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

#[no_mangle]
pub extern fn init_bot(secret: &str, body: String) -> Result<Value, String> {
    let config_file = "BlaspemyBot.toml";
    match File::open(config_file) {
        Ok(mut toml) => {
            let mut s = String::new();
            match toml.read_to_string(&mut s) {
                Ok(_) => {
                    match toml::from_str(&s) {
                        Ok(config) => {
                            let bot = Telegram::init_bot(BlaspemyBot::new, secret, config);
                            match serde_json::from_str(&body) {
                                Ok(value) => bot.parse(value),
                                Err(e) => Err(format!("Syntax error on json request: {}", e)),
                            }
                        },
                        Err(e) => Err(format!("Syntax error on Toml file: {}", e)),
                    }
                },
                Err(e) => Err(format!("Unable to read Toml file: {}", e)),
            }
        },
        Err(e) => Err(format!("File {} not found: {}", config_file, e)),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
