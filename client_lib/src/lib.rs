#![deny(warnings)]
#![deny(missing_docs)]

//! # client_lib
//!
//! Telegram bot client lib
//!
//! Provides a facility to call Telegram Bots API, a Trait for Bot structs
//! and classes for all involved entities.

#[macro_use] extern crate serde_derive;
extern crate serde;
#[macro_use] extern crate serde_json;
extern crate reqwest;
extern crate toml;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

use reqwest::multipart::Form;
use reqwest::Client;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

/// Telegram bot entities
pub mod entities;

/// Session handler
pub mod session;

/// This enum describes all possible call params
pub enum Param<'a> {
    /// a simple string value
    Value(&'a str),
    /// a file (see InputFile)
    File(entities::InputFile),
    /// a boolean flag
    Flag(bool),
    /// see ParseMode
    ParseMode(entities::ParseMode),
    /// see ReplyMarkup
    ReplyMarkup(entities::ReplyMarkup),
    /// see ChatAction
    ChatAction(entities::ChatAction),
}

/// #Telegram
/// This class defines all possible calls to Telegram bot APIs
pub struct Telegram {
    http_token: String,
    client: Client,
}

impl Telegram {
    /// given a Bot constructor, performs all mandatory checks before instancing it
    pub fn init_bot<B, F>(constructor: F, secret: &str, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<HashMap<String, JsonValue>>>) -> Result<B, String>
        where F: Fn(Telegram, &Arc<RwLock<TomlValue>>, &Arc<RwLock<HashMap<String, JsonValue>>>) -> B,
            B: Bot
    {
        (config.read().map_err(|e| format!("Error read locking configuration: {:?}", e)))
            .and_then(|cnf| {
                cnf.get("SECRET").ok_or(String::from("SECRET config value not found"))
                    .and_then(|secret_value| secret_value.as_str().ok_or(String::from("Error interpreting SECRET config value")))
                    .and_then(|cnf_secret| {
                        if secret != cnf_secret {
                            Err(String::from("Secret mismatch"))
                        }
                        else {
                            cnf.get("HTTP_TOKEN").ok_or(String::from("HTTP_TOKEN config value not found"))
                        }
                    })
                    .and_then(|token_value| token_value.as_str().ok_or(String::from("Error interpreting HTTP_TOKEN config value")))
                    .and_then(|cnf_token| Ok(constructor(Telegram::new(cnf_token), config, session)))
            })
    }

    /// internal constructor
    fn new(token: &str) -> Telegram {
        Telegram {
            http_token: token.to_owned(),
            client: Client::new(),
        }
    }

    /// #sendMessage
    /// Use this method to send text messages. On success, the sent Message is returned.
    pub fn send_message(&self, chat_id: &str, message: &str, reply_id: Option<&str>, preview: Option<bool>, parse_mode: Option<entities::ParseMode>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("text", Param::Value(message));

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match preview {
            Some(false) => {},
            _ => {
                params.insert("disable_web_page_preview", Param::Flag(true));
            },
        }

        match parse_mode {
            Some(value) => {
                params.insert("parse_mode", Param::ParseMode(value));
            },
            None => {},
        }

        match reply_markup {
            Some(value) => {
                params.insert("reply_markup", Param::ReplyMarkup(value));
            },
            None => {},
        }

        self.call_telegram("sendMessage", params)
    }

    /// #deleteMessage
    /// Use this method to delete a message, including service messages, with the following limitations:
    /// - A message can only be deleted if it was sent less than 48 hours ago.
    /// - Bots can delete outgoing messages in groups and supergroups.
    /// - Bots granted can_post_messages permissions can delete outgoing messages in channels.
    /// - If the bot is an administrator of a group, it can delete any message there.
    /// - If the bot has can_delete_messages permission in a supergroup or a channel, it can delete any message there.
    /// Returns True on success.
    pub fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("message_id", Param::Value(message_id));

        self.call_telegram("deleteMessage", params)
    }

    /// #getFile
    /// Use this method to get basic info about a file and prepare it for downloading.
    /// For the moment, bots can download files of up to 20MB in size.
    /// On success, a File object is returned.
    /// The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>,
    /// where <file_path> is taken from the response.
    /// It is guaranteed that the link will be valid for at least 1 hour.
    /// When the link expires, a new one can be requested by calling getFile again.
    pub fn get_file(&self, file_id: &str) -> Result<String, String> {
        let mut params = HashMap::new();
        params.insert("file_id", Param::Value(file_id));

        let res = self.call_telegram("getFile", params)?;

        res["result"]["file_path"].as_str().ok_or(String::from("Unable to retrieve file_path"))
            .and_then(|file_path| self.client.get(&format!("https://api.telegram.org/bot{}/{}", self.http_token, file_path)).send()
                .and_then(|mut file| file.text())
                .map_err(|e| format!("{:?}", e))
            )
    }

    /// #sendPhoto
    /// Use this method to send photos. On success, the sent Message is returned.
    pub fn send_photo(&self, chat_id: &str, photo: entities::InputFile, caption: Option<&str>, reply_id: Option<&str>, preview: Option<bool>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("photo", Param::File(photo));

        match caption {
            Some(value) => {
                params.insert("caption", Param::Value(value));
            }
            None => {},
        }

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match preview {
            Some(false) => {},
            _ => {
                params.insert("disable_web_page_preview", Param::Flag(true));
            },
        }

        match reply_markup {
            Some(value) => {
                params.insert("reply_markup", Param::ReplyMarkup(value));
            },
            None => {},
        }

        self.call_telegram("sendPhoto", params)
    }

    /// #sendAudio
    /// Use this method to send audio files, if you want Telegram clients to display them in the music player.
    /// Your audio must be in the .mp3 format.
    /// On success, the sent Message is returned.
    /// Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.
    /// For sending voice messages, use the sendVoice method instead.
    pub fn send_audio(&self, chat_id: &str, audio: entities::InputFile, duration: Option<&str>, performer: Option<&str>, title: Option<&str>, reply_id: Option<&str>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("audio", Param::File(audio));

        match duration {
            Some(value) => {
                params.insert("duration", Param::Value(value));
            },
            None => {},
        }

        match performer {
            Some(value) => {
                params.insert("performer", Param::Value(value));
            },
            None => {},
        }

        match title {
            Some(value) => {
                params.insert("title", Param::Value(value));
            },
            None => {},
        }

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match reply_markup {
            Some(value) => {
                params.insert("reply_markup", Param::ReplyMarkup(value));
            },
            None => {},
        }

        self.call_telegram("sendAudio", params)
    }

    /// #sendVoice
    /// Use this method to send audio files, if you want Telegram clients to display the file as a playable voice message.
    /// For this to work, your audio must be in an .ogg file encoded with OPUS (other formats may be sent as Audio or Document).
    /// On success, the sent Message is returned.
    /// Bots can currently send voice messages of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_voice(&self, chat_id: &str, voice: entities::InputFile, duration: Option<&str>, reply_id: Option<&str>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("voice", Param::File(voice));

        match duration {
            Some(value) => {
                params.insert("duration", Param::Value(value));
            },
            None => {},
        }

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match reply_markup {
            Some(value) => {
                params.insert("reply_markup", Param::ReplyMarkup(value));
            },
            None => {},
        }

        self.call_telegram("sendVoice", params)
    }

    /// #sendDocument
    /// Use this method to send general files.
    /// On success, the sent Message is returned.
    /// Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    pub fn send_document(&self, chat_id: &str, document: entities::InputFile, reply_id: Option<&str>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("document", Param::File(document));

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match reply_markup {
            Some(value) => {
                params.insert("reply_markup", Param::ReplyMarkup(value));
            },
            None => {},
        }

        self.call_telegram("sendDocument", params)
    }

    /// #sendChatAction
    /// Use this method when you need to tell the user that something is happening on the bot's side.
    /// The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
    /// Returns True on success.
    pub fn send_chat_action(&self, chat_id: &str, action: entities::ChatAction) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("action", Param::ChatAction(action));

        self.call_telegram("sendChatAction", params)
    }

    /// internal call facility
    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<JsonValue, String> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.http_token, method);
        self.client.post(&url)
            .multipart(Telegram::write_body(params)?)
            .send()
            .and_then(|mut response| response.json())
            .map_err(|e| format!("{:?}", e))
    }

    /// internal multipart writer
    fn write_body(params: HashMap<&str, Param>) -> Result<Form, String> {
        let mut form = Form::new();

        for (name, value) in params {
            form = match value {
                Param::Value(s) => form.text::<String, String>(name.to_owned(), s.to_owned()),
                Param::File(v) => match v {
                    entities::InputFile::File(s) => match form.file::<String, String>(name.to_owned(), s.to_owned()) {
                        Ok(f) => f,
                        Err(e) => { return Err(format!("Unable to add file field {} to request: {:?}", name, e)); },
                    },
                    entities::InputFile::FileId(s) | entities::InputFile::Url(s) => form.text::<String, String>(name.to_owned(), s.to_owned()),
                },
                Param::Flag(v) => form.text::<String, String>(name.to_owned(), String::from(if v { "true" } else { "false" })),
                Param::ParseMode(ref v) => form.text::<String, String>(name.to_owned(), v.to_string()),
                Param::ReplyMarkup(ref v) => match serde_json::to_string(v) {
                    Ok(value) => form.text::<String, String>(name.to_owned(), value.to_owned()),
                    Err(e) => { return Err(format!("Unable to add reply_markup field {} to request: {:?}", name, e)); },
                },
                Param::ChatAction(ref v) => form.text::<String, String>(name.to_owned(), v.to_string()),
            };
        }

        Ok(form)
    }
}

/// base trait for Telegram bots
pub trait Bot {
    /// creates a new instance of the Bot
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<HashMap<String, JsonValue>>>) -> Self;

    /// uses the correct method to retrieve method and arguments from Request, then dispatches it
    fn parse(&self, request: &entities::Request) -> Result<JsonValue, String> {
        let (method, args) = match request.get_type()? {
            entities::RequestType::Message => self.parse_message(request)?,
            entities::RequestType::EditedMesage => self.parse_edited_message(request)?,
            entities::RequestType::InlineQuery => self.parse_inline_query(request)?,
            entities::RequestType::ChosenInlineResult => self.parse_chosen_inline_result(request)?,
            entities::RequestType::CallbackQuery => self.parse_callback_query(request)?,
        };
        self.dispatch(&method, args, request)
    }

    /// given a Request of type Message, retrieve method ad arguments
    fn parse_message(&self, _request: &entities::Request) -> Result<((String, Vec<String>)), String> {
        Err(String::from("Not managed"))
    }

    /// given a Request of type EditedMessage, retrieve method ad arguments
    fn parse_edited_message(&self, _request: &entities::Request) -> Result<((String, Vec<String>)), String> {
        Err(String::from("Not managed"))
    }

    /// given a Request of type InlineQuery, retrieve method ad arguments
    fn parse_inline_query(&self, _request: &entities::Request) -> Result<((String, Vec<String>)), String> {
        Err(String::from("Not managed"))
    }

    /// given a Request of type InlineResult, retrieve method ad arguments
    fn parse_chosen_inline_result(&self, _request: &entities::Request) -> Result<((String, Vec<String>)), String> {
        Err(String::from("Not managed"))
    }

    /// given a Request of type CallbackQuery, retrieve method ad arguments
    fn parse_callback_query(&self, _request: &entities::Request) -> Result<((String, Vec<String>)), String> {
        Err(String::from("Not managed"))
    }

    /// given a method and it's arguments, perform the operation on the given Request
    fn dispatch(&self, method: &str, args: Vec<String>, request: &entities::Request) -> Result<JsonValue, String>;
}

#[cfg(test)]
mod tests {
    use super::{Telegram, serde_json};
    use super::serde_json::value::Value;

    #[test]
    fn it_works() {
        let client = Telegram::new("test");
        let res = client.send_message("123", "prova", None, None, None, None);

        assert_eq!(
            serde_json::from_str::<Value>("{\"ok\":false,\"error_code\":404,\"description\":\"Not Found\"}").expect("Unable to json encode test string"),
            res.expect("Failed call")
        );
    }
}
