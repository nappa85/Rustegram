extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::{Bot, Telegram};

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue,

struct BlaspemyBot {
    api: Telegram,
    config: TomlValue,
}

impl Bot for BlaspemyBot {
    fn new(api: Telegram, cfg: TomlValue) -> BlaspemyBot {
        BlaspemyBot {
            api: api,
            config: cfg,
        }
    }

    fn dispatch(&self, method: &str, json: JsonValue) -> Result<JsonValue, String> {
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
    fn about(&self, json: JsonValue) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn help(&self, json: JsonValue) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn swear(&self, json: JsonValue) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn swearto(&self, json: JsonValue) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn blackhumor(&self, json: JsonValue) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }
}

#[no_mangle]
pub extern fn init_bot(config: TomlValue, secret: &str, body: &str) -> Result<JsonValue, String> {
    let config_file = "config/BlasphemyBot.toml";
    match File::open(config_file) {
        Ok(mut toml) => {
            let mut s = String::new();
            match toml.read_to_string(&mut s) {
                Ok(_) => {
                    match toml::from_str(&s) {
                        Ok(config) => {
                            match Telegram::init_bot(BlaspemyBot::new, secret, config) {
                                Ok(bot) => match serde_json::from_str(body) {
                                    Ok(value) => bot.parse(value),
                                    Err(e) => Err(format!("Syntax error on json request: {}", e)),
                                },
                                Err(e) => Err(format!("Error during bot init: {}", e)),
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
    use super::{toml, init_bot};

    #[test]
    fn it_works() {
        assert_eq!(init_bot(config, config["SECRET"].as_str().unwrap(), "{}"), Err(String::from("TODO")));
    }
}
