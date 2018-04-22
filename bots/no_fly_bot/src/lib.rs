#![deny(warnings)]
#![deny(missing_docs)]

//! # no_fly_bot
//!
//! Telegram bot NoFlyBot
//!
//! NoFlyBot implementation

extern crate client_lib;
extern crate toml;
extern crate serde_json;

use std::sync::{Arc, RwLock};
use std::process::Command;

use client_lib::{Bot, Telegram};
use client_lib::entities::{Request, Message, ParseMode};
use client_lib::session::Session;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

struct NoFlyBot {
    api: Telegram,
    config: Arc<RwLock<TomlValue>>,
    _session: Arc<RwLock<Session>>,
}

impl Bot for NoFlyBot {
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<Session>>) -> NoFlyBot {
        NoFlyBot {
            api: api,
            config: config.clone(),
            _session: session.clone(),
        }
    }

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        self.get_command_and_arguments(request.get_message())
    }

    fn parse_edited_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        self.get_command_and_arguments(request.get_edited_message())
    }

    fn dispatch(&self, method: &str, args: Vec<String>, request: &Request) -> Result<JsonValue, String> {
        match self.config.read() {
            Ok(config) => match config["commands"].get(method) {
                Some(exe) => match exe.as_str() {
                    Some(path) => {
                        let mut new_args: Vec<String> = Vec::new();

                        let (chat_id, user_id) = match request.get_message() {
                            &Some(ref msg) => (msg.get_chat().get_id().to_string(), msg.get_from().get_id().to_string()),
                            &None => match request.get_edited_message() {
                                &Some(ref msg) => (msg.get_chat().get_id().to_string(), msg.get_from().get_id().to_string()),
                                &None => {
                                    return Err(String::from("Unsupported message type"));
                                },
                            },
                        };

                        new_args.push(chat_id.clone());
                        new_args.push(user_id);
                        for s in args {
                            new_args.push(s);
                        }

                        match Command::new(path).args(&new_args).output() {
                            Ok(out) => self.api.send_message(&chat_id, &String::from_utf8_lossy(&out.stdout), None, None, Some(ParseMode::Markdown), None),
                            Err(e) => Err(format!("Error executing {}: {:?}", method, e)),
                        }
                    },
                    None => Err(format!("Command {} incorrectly configured", method)),
                },
                None => Err(format!("Command {} not configured", method)),
            },
            Err(e) => Err(format!("Error read locking config: {:?}", e)),
        }
    }
}

impl NoFlyBot {
    fn get_command_and_arguments(&self, message: &Option<Box<Message>>) ->Result<(String, Vec<String>), String> {
        match message {
            &Some(ref msg) => {
                match msg.get_text() {
                    &Some(ref txt) => {
                        let text = txt.trim();
                        if text.starts_with('/') || text.starts_with('#') {
                            let mut words: Vec<String> = Vec::new();
                            for s in text.split(' ') {
                                words.push(String::from(s));
                            }

                            //remove first char
                            let method = words.swap_remove(0);
                            let mut chars = method.chars();
                            chars.next();

                            Ok((String::from(chars.as_str()), words))
                        }
                        else {
                            Err(format!("String \"{}\" doesn't contains a command", text))
                        }
                    },
                    &None => match msg.get_location() {
                        &Some(ref loc) => Ok((String::from("set_location"), vec![loc.get_longitude().to_string(), loc.get_latitude().to_string()])),
                        &None => Err(String::from("Unsupported message type")),
                    }
                }
            },
            &None => Err(String::from("Empty message")),
        }
    }
}

/// public C ABI to call the bot
#[no_mangle]
pub extern fn init_bot(ptr_config: *const Arc<RwLock<TomlValue>>, ptr_session: *const Arc<RwLock<Session>>, secret: &str, ptr_request: *const &Request) -> *const Result<JsonValue, String> {
    let config = unsafe {
        assert!(!ptr_config.is_null());
        &*ptr_config
    };
    let session = unsafe {
        assert!(!ptr_session.is_null());
        &*ptr_session
    };
    let request = unsafe {
        assert!(!ptr_request.is_null());
        &*ptr_request
    };

    Box::into_raw(Box::new(match Telegram::init_bot(NoFlyBot::new, secret, &config, &session) {
        Ok(bot) => bot.parse(request),
        Err(e) => Err(format!("Error during bot init: {}", e)),
    }))
}

#[cfg(test)]
mod tests {
    use super::{toml, serde_json, init_bot};
    use super::client_lib::entities::Request;
    use super::client_lib::session::Session;
    use std::sync::{Arc, RwLock};

    #[test]
    fn it_works() {
        let config: toml::Value = toml::from_str(r#"SECRET = "prova"
HTTP_TOKEN = "test"

[command]
set_position = "echo"
find = "echo"
"#).unwrap();

        let messages = [r#"{
"update_id":10000,
"message":{
  "date":1441645532,
  "chat":{
     "last_name":"Test Lastname",
     "id":1111111,
     "type": "private",
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "message_id":1365,
  "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
  },
  "text":"/find pippo"
}
}"#, r#"{
  "update_id":241066349,
  "edited_message":{
    "message_id":4,
    "from":{
    "is_bot": true,
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "chat":{
     "last_name":"Test Lastname",
     "type": "private",
     "id":1111111,
     "first_name":"Test Firstname",
     "username":"Testusername"
    },
    "date":1520764899,
    "edit_date":1520764972,
    "location":{
      "latitude":45.558900,
      "longitude":12.233439
    }
  }
}"#];
        for s in messages.iter() {
            let request: Request = serde_json::from_str(s).unwrap();

            assert_eq!(init_bot(Box::into_raw(Box::new(Arc::new(RwLock::new(config)))), Box::into_raw(Box::new(Arc::new(RwLock::new(Session::new())))), "prova", Box::into_raw(Box::new(request))), Err(String::from("TODO")));
        }
    }
}
