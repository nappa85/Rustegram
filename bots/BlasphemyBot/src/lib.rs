extern crate client_lib;
extern crate toml;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use client_lib::{Bot, Telegram};
use client_lib::entities::Request;

use serde_json::value::Value as JsonValue;

use toml::Value as TomlValue;

struct BlaspemyBot {
    api: Telegram,
    config: Arc<RwLock<TomlValue>>,
    session: Arc<RwLock<HashMap<String, JsonValue>>>,
}

impl Bot for BlaspemyBot {
    fn new(api: Telegram, config: &Arc<RwLock<TomlValue>>, session: &Arc<RwLock<HashMap<String, JsonValue>>>) -> BlaspemyBot {
        BlaspemyBot {
            api: api,
            config: config.clone(),
            session: session.clone(),
        }
    }

    fn parse_message(&self, request: &Request) -> Result<(String, Vec<String>), String> {
//         if request.message.text.is_none() {
//             return Err(String::from("Command not found"));
//         }

        Ok((String::from("swear"), Vec::new()))
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

impl BlaspemyBot {
    fn about(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("about command"))
    }

    fn help(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("help command"))
    }

    fn swear(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("swear command"))
    }

    fn swearto(&self, request: &Request, args: Vec<String>) -> Result<JsonValue, String> {
        Err(String::from("swearto command"))
    }

    fn blackhumor(&self, request: &Request) -> Result<JsonValue, String> {
        Err(String::from("blackhumor command"))
    }
}

#[no_mangle]
pub extern fn init_bot(ptr_config: *const Arc<RwLock<TomlValue>>, ptr_session: *const Arc<RwLock<HashMap<String, JsonValue>>>, secret: &str, ptr_request: *const Request) -> Result<JsonValue, String> {
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

    match Telegram::init_bot(BlaspemyBot::new, secret, &config, &session) {
        Ok(bot) => bot.parse(request),
        Err(e) => Err(format!("Error during bot init: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::{toml, serde_json, init_bot};
    use super::client_lib::entities::Request;
    use std::collections::HashMap;
    use std::sync::{Arc, RwLock};

    #[test]
    fn it_works() {
        let config: toml::Value = toml::from_str(r#"SECRET = "prova"
HTTP_TOKEN = "test""#).unwrap();
        let session = Arc::new(RwLock::new(HashMap::new()));
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
