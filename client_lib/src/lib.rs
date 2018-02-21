// #[macro_use]
// extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate toml;

use std::collections::HashMap;

use reqwest::multipart::Form;
use reqwest::Client;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

pub mod entities;

//pub mod session;

pub enum Param<'a> {
    Value(&'a str),
    File(entities::InputFile),
    Flag(bool),
    ParseMode(entities::ParseMode),
    ReplyMarkup(entities::ReplyMarkup),
    ChatAction(entities::ChatAction),
}

pub struct Telegram {
    http_token: String,
    client: Client,
}

impl Telegram {
    pub fn init_bot<B: Bot, F>(constructor: F, secret: &str, config: TomlValue) -> Result<B, String>
        where F: Fn(Telegram, TomlValue) -> B
    {
        match config["SECRET"].as_str() {
            Some(conf_secret) => {
                if secret == conf_secret {
                    match config["HTTP_TOKEN"].as_str() {
                        Some(token) => Ok(constructor(Telegram::new(token), config.clone())),
                        None => Err(String::from("Error interpreting HTTP_TOKEN config value")),
                    }
                }
                else {
                    Err(String::from("Secret mismatch"))
                }
            },
            None => Err(String::from("Error interpreting SECRET config value")),
        }
    }

    fn new(token: &str) -> Telegram {
        Telegram {
            http_token: token.to_owned(),
            client: Client::new(),
        }
    }

    pub fn send_message(&self, chat_id: &str, message: &str, reply_id: Option<&str>, preview: Option<bool>, parse_mode: Option<entities::ParseMode>, reply_markup: Option<entities::ReplyMarkup>) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("message", Param::Value(message));

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

    pub fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("message_id", Param::Value(message_id));

        self.call_telegram("deleteMessage", params)
    }

    pub fn get_file(&self, file_id: &str) -> Result<String, String> {
        let mut params = HashMap::new();
        params.insert("file_id", Param::Value(file_id));

        let res = self.call_telegram("getFile", params)?;

        match res["result"]["file_path"].as_str() {
            Some(file_path) => {
                let url = format!("https://api.telegram.org/bot{}/{}", self.http_token, file_path);
                match self.client.get(&url).send() {
                    Ok(mut file) => {
                        match file.text() {
                            Ok(file) => Ok(file),
                            Err(e) => Err(format!("{}", e)),
                        }
                    },
                    Err(e) => Err(format!("{}", e)),
                }
            },
            None => Err(String::from("Unable to retrieve file_path")),
        }
    }

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

    pub fn send_chat_action(&self, chat_id: &str, action: entities::ChatAction) -> Result<JsonValue, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("action", Param::ChatAction(action));

        self.call_telegram("sendChatAction", params)
    }

    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<JsonValue, String> {
        let url = format!("https://api.telegram.org/bot{}/{}", self.http_token, method);
        match self.client.post(&url)
            .multipart(Telegram::write_body(params)?)
            .send() {
            Ok(mut response) => {
                match response.json() {
                    Ok(res) => Ok(res),
                    Err(e) => Err(format!("{}", e)),
                }
            },
            Err(e) => Err(format!("{}", e)),
        }
    }

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
                Param::Flag(ref v) => match serde_json::to_string(v) {
                    Ok(value) => form.text::<String, String>(name.to_owned(), value.to_owned()),
                    Err(e) => { return Err(format!("Unable to add flag field {} to request: {:?}", name, e)); },
                },
                Param::ParseMode(ref v) => match serde_json::to_string(v) {
                    Ok(value) => form.text::<String, String>(name.to_owned(), value.to_owned()),
                    Err(e) => { return Err(format!("Unable to add parse_mode field {} to request: {:?}", name, e)); },
                },
                Param::ReplyMarkup(ref v) => match serde_json::to_string(v) {
                    Ok(value) => form.text::<String, String>(name.to_owned(), value.to_owned()),
                    Err(e) => { return Err(format!("Unable to add reply_markup field {} to request: {:?}", name, e)); },
                },
                Param::ChatAction(ref v) => match serde_json::to_string(v) {
                    Ok(value) => form.text::<String, String>(name.to_owned(), value.to_owned()),
                    Err(e) => { return Err(format!("Unable to add chat_action field {} to request: {:?}", name, e)); },
                },
            };
        }

        Ok(form)
    }
}

pub trait Bot {
    fn new(api: Telegram, cfg: TomlValue) -> Self;

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

    fn parse_message(&self, request: &entities::Request) -> Result<(String, Vec<String>), String>;

    fn parse_edited_message(&self, request: &entities::Request) -> Result<(String, Vec<String>), String>;

    fn parse_inline_query(&self, request: &entities::Request) -> Result<(String, Vec<String>), String>;

    fn parse_chosen_inline_result(&self, request: &entities::Request) -> Result<(String, Vec<String>), String>;

    fn parse_callback_query(&self, request: &entities::Request) -> Result<(String, Vec<String>), String>;

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
