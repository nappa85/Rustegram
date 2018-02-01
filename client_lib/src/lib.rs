extern crate reqwest;
extern crate serde_json;
extern crate ini;

use std::collections::HashMap;
use std::io::Read;
use reqwest::multipart::Form;
use reqwest::{Client, Result};
use serde_json::value::Value;
use ini::Ini;

pub enum Param<'a> {
    Value(&'a str),
    File(&'a str),
    Flag(bool),
    Parse(ParseMode),
    Markup(Keyboard),
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
        "{\"force_reply\":".to_owned() + (if self.force_reply { "true" } else { "false" }) + ",\"selective\":" + (if self.force_reply { "true" } else { "false" }) + "}"
    }
}

pub struct Telegram {
    config: Ini,
    client: Client,
}

impl Telegram {
    pub fn new(cnf: Ini) -> Telegram {
        Telegram {
            config: cnf,
            client: Client::new(),
        }
    }

    pub fn send_message(&self, chat_id: &str, message: &str, reply_id: Option<&str>, force_reply: Option<bool>, preview: Option<bool>, parse_mode: Option<ParseMode>, keyboard: Option<Keyboard>) -> Result<Value> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id));
        params.insert("message", Param::Value(message));

        if !reply_id.is_none() {
            params.insert("reply_id", Param::Value(reply_id.unwrap()));
        }

        if force_reply == Some(true) {
            params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
        }

        if preview.is_none() || preview == Some(false) {
            params.insert("disable_web_page_preview", Param::Flag(true));
        }

        if !parse_mode.is_none() {
            params.insert("parse_mode", Param::Parse(parse_mode.unwrap()));
        }

        if !keyboard.is_none() {
            params.insert("reply_markup", Param::Markup(keyboard.unwrap()));
        }

        self.call_telegram("sendMessage", params)
    }

    pub fn get_file(&self, file_id: &str) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("file_id", Param::Value(file_id));

        let res = self.call_telegram("getFile", params)?;

        let url = "https://api.telegram.org/bot".to_owned() + self.config.get_from::<String>(None, "HTTP_TOKEN").expect("Can't find HTTP_TOKEN in your config") + "/" + res["result"]["file_path"].as_str().unwrap();
        let mut file = self.client.get(&url).send()?;

        file.text()
    }

    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<Value> {
        let url = "https://api.telegram.org/bot".to_owned() + self.config.get_from::<String>(None, "HTTP_TOKEN").expect("Can't find HTTP_TOKEN in your config") + "/" + method;
        let mut response = self.client.post(&url)
            .multipart(Telegram::write_body(params))
            .send()?;

        response.json()
    }

    fn write_body(params: HashMap<&str, Param>) -> Form {
        let mut form = Form::new();

        for (name, value) in params {
            form = match value {
                Param::Value(s) => form.text::<String, String>(name.to_owned(), s.to_owned()),
                Param::File(s) => form.file::<String, String>(name.to_owned(), s.to_owned()).unwrap(),
                Param::Flag(b) => form.text::<String, String>(name.to_owned(), (if b { "true" } else { "false" }).to_owned()),
                Param::Parse(ref p) => form.text::<String, String>(name.to_owned(), p.to_string()),
                Param::Markup(ref k) => form.text::<String, String>(name.to_owned(), k.to_string()),
            };
        }

        form
    }
}

#[cfg(test)]
mod tests {
    use super::{Ini, Telegram, serde_json};

    #[test]
    fn it_works() {
        let mut conf = Ini::new();
        conf.set_to::<String>(None, "HTTP_TOKEN".to_owned(), "test".to_owned());

        let client = Telegram::new(conf);
        let res = client.send_message("123", "prova", None, None, None, None, None);

        assert_eq!(serde_json::from_str::<serde_json::value::Value>("{\"ok\":false,\"error_code\":404,\"description\":\"Not Found\"}").unwrap(), res.unwrap());
    }
}
