#![deny(warnings)]
#![deny(missing_docs)]

//! # no_fly_bot
//!
//! Telegram bot PoGoVe
//!
//! PoGoVe implementation

extern crate client_lib;
extern crate toml;
extern crate serde_json;

use std::sync::{Arc, RwLock};
use std::process::Command;

use client_lib::{Bot, Telegram};
use client_lib::entities::{Request, Message, ParseMode, ReplyMarkup};
use client_lib::session::Session;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

struct PoGoVe {
    api: Telegram,
    config: Arc<RwLock<TomlValue>>,
    session: Arc<RwLock<Session>>,
}

impl Bot for PoGoVe {
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<Session>>) -> NoFlyBot {
        PoGoVe {
            api: api,
            config: config.clone(),
            session: session.clone(),
        }
    }

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        request.get_message().ok_or(String::from("Unsupported message type"))
            .and_then(|ref msg| msg.get_new_chat_members().ok_or(String::from("Unsupported message type")))
            .and_then(|_| Ok((String::from("greet"), Vec::new())))
    }

    fn dispatch(&self, method: &str, args: Vec<String>, request: &Request) -> Result<JsonValue, String> {
        match method {
            "about" => self.about(request),
            "help" => self.help(request),
            "greet" => self.greet(request),
            _ => Err(format!("Method {} not found", method)),
        }
    }
}

impl PoGoVe {
    fn about(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                "This bot aims to be an help in greeting and sorting people who wants to join Venice area Pokémon GO groups",
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    fn help(&self, request: &Request) -> Result<JsonValue, String> {
        match request.get_message() {
            &Some(ref msg) => self.api.send_message(
                &msg.get_chat().get_id().to_string(),
                "Actually this bot doesn't accepts commands",
                None, None, None, None),
            &None => Err(String::from("Empty message")),
        }
    }

    fn greet(&self, request: &Request) -> Result<JsonValue, String> {
        request.get_message().ok_or(String::from("Unsupported message type"))
            .and_then(|ref msg| msg.get_new_chat_members().ok_or(String::from("Unsupported message type"))
                .and_then(|ref user| self.api.send_message(
                    &msg.get_chat().get_id().to_string(),
                    &format!("Hello, {}, follow me into the white rabbit's hole", msg.get_from()),
                    None, None, ParseMode::Markdown,
                    ReplyMarkup::inline_keyboard(vec![vec![
                        InlineKeyboardButton::new(String::from("Comincia"), None, None, None, Some(String::from("it")), None, None),
                        InlineKeyboardButton::new(String::from("Start"), None, None, None, Some(String::from("en")), None, None),
                    ]])))
            )
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

    Box::into_raw(Box::new(match Telegram::init_bot(PoGoVeBot::new, secret, &config, &session) {
        Ok(bot) => bot.parse(request),
        Err(e) => Err(format!("Error during bot init: {}", e)),
    }))
}
