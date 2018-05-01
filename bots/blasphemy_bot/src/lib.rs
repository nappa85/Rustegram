#![deny(warnings)]
#![deny(missing_docs)]

//! # blasphemy_bot
//!
//! Telegram bot BlasphemyBot
//!
//! BlasphemyBot implementation

extern crate client_lib;
extern crate toml;
#[macro_use] extern crate serde_json;
extern crate rand;

use std::sync::{Arc, RwLock};

use client_lib::{Bot, Telegram};
use client_lib::entities::{Message, Request, ReplyMarkup};
use client_lib::session::Session;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

use rand::{thread_rng, Rng};

struct BlasphemyBot {
    api: Telegram,
    config: Arc<RwLock<TomlValue>>,
    session: Arc<RwLock<Session>>,
}

impl Bot for BlasphemyBot {
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<Session>>) -> BlasphemyBot {
        BlasphemyBot {
            api: api,
            config: config.clone(),
            session: session.clone(),
        }
    }

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        match request.get_message() {
            &Some(ref msg) => {
                match msg.get_text() {
                    &Some(ref txt) => {
                        let text = txt.trim();
                        if text.starts_with('/') {
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
                            (self.session.read().map_err(|e| format!("Unable to read lock session: {:?}", e)))
                                .and_then(|session| session.get(&BlasphemyBot::get_session_key(msg)).ok_or(format!("String \"{}\" doesn't contains a command", text)))
                                .and_then(|json| json.as_str().ok_or(format!("Session value mismatch: {:?}", json))
                                    .and_then(|value| {
                                        let mut words: Vec<String> = Vec::new();
                                        for s in text.split(' ') {
                                            words.push(String::from(s));
                                        }

                                        Ok((value.to_string(), words))
                                    }))
                        }
                    },
                    &None => Err(String::from("Unsupported message type")),
                }
            },
            &None => Err(String::from("Empty message")),
        }
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

impl BlasphemyBot {
    fn about(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                "This bot can help you when you need to swear but you're out of words.\n\nDeveloped by @Nappa85 under GPLv4\nSource code: https://github.com/nappa85/Rustegram",
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    fn help(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                "/swear - A generic swear\n\n/swearto - Swear about your favourite subject\nYou can pass an inline argument, or call the command and insert the subject when asked.\nFor example:\n/swearto the developer of @BlasphemyBot\n\n/blackhumor - Some good old black humor\n\n/suggest - Suggest an improvement to the developer\nYou can pass an inline argument, or call the command and insert the subject when asked.\nFor example:\n/suggest I have a new blackhumor line for you!",
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    fn swear(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                &format!("{} {} {}", self.get_random_word_a()?, self.get_random_word_b()?, self.get_random_word_c()?),
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    //TODO: session
    fn swearto(&self, request: &Request, args: Vec<String>) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => {
                if args.len() == 0 {
                    return (self.session.write().map_err(|e| format!("Unable to write lock session: {:?}", e)))
                        .and_then(|mut session| {
                            session.set(&BlasphemyBot::get_session_key(msg), json!("swearto"));

                            self.api.send_message(
                                &msg.get_chat().get_id().to_string(),
                                "Now insert a Subject for the swear",
                                Some(&msg.get_id().to_string()), None, None,
                                Some(ReplyMarkup::force_reply(true, Some(true))))
                        });
                }

                let mut text = String::new();
                for s in args {
                    text = format!("{} {}", text, s)
                }

                self.api.send_message(
                    &msg.get_chat().get_id().to_string(),
                    &format!("{}{} {}", self.get_random_word_a()?, text, self.get_random_word_c()?),
                    None, None, None, None)
            },
            &None => Err(String::from("Empty message")),
        }
    }

    fn blackhumor(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                &self.get_random_black_humor()?,
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    fn get_random_word_a(&self) -> Result<String, String> {
        self.get_random("WordsA")
    }

    fn get_random_word_b(&self) -> Result<String, String> {
        self.get_random("WordsB")
    }

    fn get_random_word_c(&self) -> Result<String, String> {
        self.get_random("WordsC")
    }

    fn get_random_black_humor(&self) -> Result<String, String> {
        self.get_random("BlacHumor")
    }

    fn get_random(&self, key: &str) -> Result<String, String> {
        (self.config.read().map_err(|e| format!("Unable to read config: {:?}", e)))
            .and_then(|config| (config[key].as_array().ok_or(format!("{} is not an array", key)))
                .and_then(|values| {
                    let mut rng = thread_rng();
                    let index = rng.gen_range::<usize>(0, values.len() - 1);
                    values[index].as_str().ok_or(format!("{}[{}] is not a string", key, index))
                })
                .and_then(|s| Ok(s.to_owned()))
            )
    }

    fn get_session_key(msg: &Message) -> String {
        msg.get_from().get_id().to_string() + "." + &msg.get_chat().get_id().to_string()
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

    Box::into_raw(Box::new(match Telegram::init_bot(BlasphemyBot::new, secret, &config, &session) {
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
HTTP_TOKEN = "test""#).unwrap();
        let session = Arc::new(RwLock::new(Session::new()));
        let request: Request = serde_json::from_str(r#"{
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
  "text":"/start"
}
}"#).unwrap();

        assert_eq!(init_bot(Box::into_raw(Box::new(Arc::new(RwLock::new(config)))), Box::into_raw(Box::new(session)), "prova", Box::into_raw(Box::new(request))), Err(String::from("TODO")));
    }
}
