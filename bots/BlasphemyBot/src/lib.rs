extern crate client_lib;
extern crate toml;
extern crate serde_json;

use client_lib::{Bot, Telegram};
use client_lib::entities::Request;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

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

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
//         if request.message.text.is_none() {
//             return Err(String::from("Command not found"));
//         }

        Ok((String::from("swear"), Vec::new()))
    }

    fn parse_edited_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        Err(String::from("Not managed"))
    }

    fn parse_inline_query(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        Err(String::from("Not managed"))
    }

    fn parse_chosen_inline_result(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        Err(String::from("Not managed"))
    }

    fn parse_callback_query(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        Err(String::from("Not managed"))
    }

    fn dispatch(&self, method: &str, args: Vec<String>, request: &Request) -> Result<JsonValue, String> {
        match method {
            "about" => self.about(request),
            "help" => self.help(request),
            "swear" => self.swear(request),
            "swearto" => self.swearto(request, args),
            "blackhumor" => self.blackhumor(request),
            _ => Err(format!("Method {} not found", method)),
        }
    }
}

impl BlaspemyBot {
    fn about(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn help(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn swear(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn swearto(&self, request: &Request, args: Vec<String>) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }

    fn blackhumor(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("TODO"))
    }
}

#[no_mangle]
pub extern fn init_bot(ptr_config: *const TomlValue, secret: &str, ptr_request: *const Request) -> Result<JsonValue, String> {
    let config = unsafe {
        assert!(!ptr_config.is_null());
        &*ptr_config
    };
    let request = unsafe {
        assert!(!ptr_request.is_null());
        &*ptr_request
    };

    match Telegram::init_bot(BlaspemyBot::new, secret, config.clone()) {
        Ok(bot) => bot.parse(request),
        Err(e) => Err(format!("Error during bot init: {}", e)),
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
