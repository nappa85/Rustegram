extern crate hyper;
extern crate multipart;
extern crate ini;

use std::collections::HashMap;
use std::io::Read;
use hyper::client::Request;
use hyper::method::Method;
use hyper::net::Streaming;
use hyper::Result;
use multipart::client::Multipart;
use ini::Ini;

pub enum Param {
    Value(String),
    File(String),
    Flag(bool),
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
        "TODO: implement this".to_owned()
    }
}

pub struct Telegram {
    config: Ini,
}

impl Telegram {
    pub fn new(cnf: Ini) -> Telegram {
        Telegram { config: cnf }
    }

    pub fn send_message(&self, chat_id: &str, message: &str, reply_id: Option<&str>, force_reply: Option<bool>, preview: Option<bool>, parse_mode: Option<ParseMode>, keyboard: Option<Keyboard>) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("chat_id", Param::Value(chat_id.to_owned()));
        params.insert("message", Param::Value(message.to_owned()));

        if !reply_id.is_none() {
            params.insert("reply_id", Param::Value(reply_id.unwrap().to_owned()));
        }

        if force_reply == Some(true) {
            params.insert("reply_markup", Param::Markup(Keyboard { force_reply: true, selective: true }));
        }

        if preview.is_none() || preview == Some(false) {
            params.insert("disable_web_page_preview", Param::Flag(true));
        }

        if !parse_mode.is_none() {
            params.insert("parse_mode", Param::Value(parse_mode.unwrap().to_string()));
        }

        if !keyboard.is_none() {
            params.insert("reply_markup", Param::Markup(keyboard.unwrap()));
        }

        self.call_telegram("sendMessage", params)
    }

    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<String> {
        let uri = ("https://api.telegram.org/bot".to_owned() + self.config.get_from::<String>(None, "HTTP_TOKEN").expect("Can't find HTTP_TOKEN in your config") + "/" + method).parse().expect("Can't parse Telegram API address");

        let request = Request::new(Method::Post, uri)?;

        let mut multipart = Multipart::from_request(request)?;

        Telegram::write_body(&mut multipart, params)?;

        let mut response = multipart.send()?;

        let mut res = String::new();
        response.read_to_string(&mut res).expect("Failed to read response");

        Ok(res)
    }

    fn write_body(multi: &mut Multipart<Request<Streaming>>, params: HashMap<&str, Param>) -> Result<()> {
        for (name, value) in &params {
            match value {
                &Param::Value(ref s) => multi.write_text(name, s)?,
                &Param::File(ref s) => multi.write_file(name, s)?,
                &Param::Flag(b) => multi.write_file(name, if b { "true" } else { "false" })?,
                &Param::Markup(ref k) => multi.write_file(name, k.to_string())?,
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Ini, Telegram};

    #[test]
    fn it_works() {
        let mut conf = Ini::new();
        conf.set_to::<String>(None, "HTTP_TOKEN".to_owned(), "test".to_owned());

        let client = Telegram::new(conf);
        let res = client.send_message("123", "prova", None, None, None, None, None);
        
        println!("response: {}", res.unwrap());

        assert_eq!(2 + 2, 4);
    }
}
