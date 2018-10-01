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
extern crate parking_lot;

use std::sync::Arc;

use client_lib::{Bot, Telegram};
use client_lib::entities::{Request, ParseMode, ReplyMarkup, InlineKeyboardButton};
use client_lib::session::Session;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

use parking_lot::RwLock;


struct PoGoVe {
    api: Telegram,
    _config: Arc<RwLock<TomlValue>>,
    _session: Arc<RwLock<Session>>,
}

impl Bot for PoGoVe {
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<Session>>) -> PoGoVe {
        PoGoVe {
            api: api,
            _config: config.clone(),
            _session: session.clone(),
        }
    }

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
        match request.get_message() {
            Some(ref msg) => match msg.get_new_chat_members() {
                Some(_) => Ok((String::from("greet"), Vec::new())),
                None => Err(String::from("Unsupported message type")),
            },
            None => Err(String::from("Unsupported message type")),
        }
    }

    fn dispatch(&self, method: &str, _: Vec<String>, request: &Request) -> Result<JsonValue, String> {
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
        match request.get_message() {
            Some(ref msg) => match msg.get_new_chat_members() {
                Some(ref users) => self.api.send_message(
                    &msg.get_chat().get_id().to_string(),
                    &format!("Hello,{}, follow me into the white rabbit's hole", users.iter().fold(String::new(), |acc, user| {
                        acc + " " + &user.get_link(ParseMode::Markdown)
                    })),
                    None, None, Some(ParseMode::Markdown),
                    Some(ReplyMarkup::inline_keyboard(vec![vec![
                        InlineKeyboardButton::new(String::from("Comincia"), None, None, None, Some(String::from("it")), None, None),
                        InlineKeyboardButton::new(String::from("Start"), None, None, None, Some(String::from("en")), None, None),
                    ]]))),
                None => Err(String::from("Unsupported message type")),
            },
            None => Err(String::from("Unsupported message type")),
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

    Box::into_raw(Box::new(match Telegram::init_bot(PoGoVe::new, secret, &config, &session) {
        Ok(bot) => bot.parse(request),
        Err(e) => Err(format!("Error during bot init: {}", e)),
    }))
}
