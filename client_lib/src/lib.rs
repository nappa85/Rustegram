#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate serde_json;
extern crate toml;

use std::collections::HashMap;

use reqwest::multipart::Form;
use reqwest::Client;
use serde_json::value::Value;

pub mod session;

pub enum Param<'a> {
    Value(&'a str),
    File(&'a str),
    Flag(bool),
    Parse(ParseMode),
    Markup(Keyboard),
    Action(ChatAction),
}

pub enum ParseMode {
    Markdown,
    HTML,
}

impl ToString for ParseMode {
    fn to_string(&self) -> String {
        match *self {
            ParseMode::Markdown => "Markdown".to_owned(),
            ParseMode::HTML => "HTML".to_owned(),
        }
    }
}

pub struct Keyboard {
    force_reply: bool,
    selective: bool,
}

impl ToString for Keyboard {
    fn to_string(&self) -> String {
        "{\"force_reply\":".to_owned() + (if self.force_reply { "true" } else { "false" }) + ",\"selective\":" + (if self.selective { "true" } else { "false" }) + "}"
    }
}

pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

impl ToString for ChatAction {
    fn to_string(&self) -> String {
        (match *self {
            ChatAction::Typing => "typing",
            ChatAction::UploadPhoto => "upload_photo",
            ChatAction::RecordVideo => "record_video",
            ChatAction::UploadVideo => "upload_video",
            ChatAction::RecordAudio => "record_audio",
            ChatAction::UploadAudio => "upload_audio",
            ChatAction::UploadDocument => "upload_document",
            ChatAction::FindLocation => "find_location",
        }).to_owned()
    }
}

pub struct Telegram {
    http_token: String,
    client: Client,
}

impl Telegram {
    pub fn init_bot<B: Bot, F>(constructor: F, secret: &str, config: toml::Value) -> Result<B, String>
        where F: Fn(Telegram, toml::Value) -> B
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

    pub fn send_message(&self, chat_id: &str, message: &str, reply_id: Option<&str>, force_reply: Option<bool>, preview: Option<bool>, parse_mode: Option<ParseMode>, keyboard: Option<Keyboard>) -> Result<Value, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("message", Param::Value(message));

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match force_reply {
            Some(true) => {
                params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
            },
            _ => {},
        }

        match preview {
            Some(true) => {},
            _ => {
                params.insert("disable_web_page_preview", Param::Flag(true));
            },
        }

        match parse_mode {
            Some(value) => {
                params.insert("parse_mode", Param::Parse(value));
            },
            None => {},
        }

        match keyboard {
            Some(value) => {
                params.insert("reply_markup", Param::Markup(value));
            },
            None => {},
        }

        self.call_telegram("sendMessage", params)
    }

    pub fn delete_message(&self, chat_id: &str, message_id: &str) -> Result<Value, String> {
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

    pub fn send_photo(&self, chat_id: &str, photo: &str, caption: Option<&str>, reply_id: Option<&str>, force_reply: Option<bool>, preview: Option<bool>) -> Result<Value, String> {
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

        match force_reply {
            Some(true) => {
                params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
            },
            _ => {},
        }

        match preview {
            Some(true) => {},
            _ => {
                params.insert("disable_web_page_preview", Param::Flag(true));
            },
        }

        self.call_telegram("sendPhoto", params)
    }

    pub fn send_audio(&self, chat_id: &str, audio: &str, duration: Option<&str>, performer: Option<&str>, title: Option<&str>, reply_id: Option<&str>, force_reply: Option<bool>) -> Result<Value, String> {
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

        match force_reply {
            Some(true) => {
                params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
            },
            _ => {},
        }

        self.call_telegram("sendAudio", params)
    }

    pub fn send_voice(&self, chat_id: &str, voice: &str, duration: Option<&str>, reply_id: Option<&str>, force_reply: Option<bool>) -> Result<Value, String> {
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

        match force_reply {
            Some(true) => {
                params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
            },
            _ => {},
        }

        self.call_telegram("sendVoice", params)
    }

    pub fn send_document(&self, chat_id: &str, document: &str, reply_id: Option<&str>, force_reply: Option<bool>) -> Result<Value, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("document", Param::File(document));

        match reply_id {
            Some(value) => {
                params.insert("reply_to_message_id", Param::Value(value));
            },
            None => {},
        }

        match force_reply {
            Some(true) => {
                params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
            },
            _ => {},
        }

        self.call_telegram("sendDocument", params)
    }

    pub fn send_chat_action(&self, chat_id: &str, action: ChatAction) -> Result<Value, String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("action", Param::Action(action));

        self.call_telegram("sendChatAction", params)
    }

    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<Value, String> {
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
                Param::File(s) => match form.file::<String, String>(name.to_owned(), s.to_owned()) {
                    Err(e) => { return Err(format!("Unable to add file to request: {}", e)); },
                    Ok(f) => f,
                },
                Param::Flag(b) => form.text::<String, String>(name.to_owned(), (if b { "true" } else { "false" }).to_owned()),
                Param::Parse(ref p) => form.text::<String, String>(name.to_owned(), p.to_string()),
                Param::Markup(ref k) => form.text::<String, String>(name.to_owned(), k.to_string()),
                Param::Action(ref a) => form.text::<String, String>(name.to_owned(), a.to_string()),
            };
        }

        Ok(form)
    }
}

pub trait Bot {
    fn new(api: Telegram, cfg: toml::Value) -> Self;

    fn parse(&self, json: Value) -> Result<Value, String> {
        Err(String::from("TODO"))
    }

    fn dispatch(&self, method: &str, json: Value) -> Result<Value, String>;
}

#[cfg(test)]
mod tests {
    use super::{Telegram, serde_json};

    #[test]
    fn it_works() {
        let client = Telegram::new("test");
        let res = client.send_message("123", "prova", None, None, None, None, None);

        assert_eq!(
            serde_json::from_str::<serde_json::value::Value>("{\"ok\":false,\"error_code\":404,\"description\":\"Not Found\"}").expect("Unable to json encode test string"),
            res.expect("Failed call")
        );
    }
}
