extern crate hyper;
extern crate hyper_native_tls;
extern crate multipart;
extern crate ini;

use std::collections::HashMap;
use std::io::Read;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::client::Request;
use hyper::Url;
// use hyper::method::Method;
use hyper::net::Streaming;
use hyper::Result;
use multipart::client::lazy::Multipart;
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

    fn call_telegram(&self, method: &str, params: HashMap<&str, Param>) -> Result<String> {
        let uri:Url = ("https://api.telegram.org/bot".to_owned() + self.config.get_from::<String>(None, "HTTP_TOKEN").expect("Can't find HTTP_TOKEN in your config") + "/" + method).parse().expect("Can't parse Telegram API address");

        //let request = Request::new(Method::Post, uri)?;

        //let mut multipart = Multipart::from_request(request)?;

        let mut multipart = Multipart::new();

        Telegram::write_body(&mut multipart, params)?;

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let mut response = multipart.client_request(&client, uri).expect("Error sending multipart request");

        let mut res = String::new();
        response.read_to_string(&mut res).expect("Failed to read response");

        Ok(res)
    }

    fn write_body<'a>(multi: &mut Multipart<'a, 'a>, params: HashMap<&'a str, Param<'a>>) -> Result<()> {
        for (name, value) in &params {
            match value {
                &Param::Value(s) => multi.add_text::<&'a str, &'a str>(name, s),
                &Param::File(s) => multi.add_file::<&'a str, &'a str>(name, s),
                &Param::Flag(b) => multi.add_text::<&'a str, &'a str>(name, if b { "true" } else { "false" }),
                &Param::Parse(ref p) => multi.add_text::<&'a str, String>(name, p.to_string()),
                &Param::Markup(ref k) => multi.add_text::<&'a str, String>(name, k.to_string()),
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

        assert_eq!("{\"ok\":false,\"error_code\":404,\"description\":\"Not Found\"}", res.unwrap());
    }
}
