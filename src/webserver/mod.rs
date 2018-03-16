extern crate hyper;
extern crate futures;
extern crate regex;
extern crate serde_json;
extern crate client_lib;

use std::sync::{Arc, Mutex};

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::regex::Regex;

use self::client_lib::entities::Request as TelegramRequest;

mod registry;

//singleton
lazy_static! {
    static ref REGISTRY: Arc<Mutex<registry::PluginRegistry>> = Arc::new(Mutex::new(registry::PluginRegistry::new()));
}

pub struct WebServer;

impl WebServer {
    /// Dispatchs the call to the right bot, if possible
    fn map_body(bot: String, secret: String, chunks: Vec<u8>) -> Response {
        //acquire a reference to bot registry
        let reg = REGISTRY.clone();

        //convert chunks to String
        match (String::from_utf8(chunks).map_err(|e| format!("Unable to convert request body to string: {}", e)))
            //convert request to struct Request
            .and_then(|body| serde_json::from_str::<TelegramRequest>(&body).map_err(|e| format!("Syntax error on json request: {}", e)))
            .and_then(|ref request|
                //lock bot registry
                (reg.lock().map_err(|e| format!("Unable to lock plugin registry: {}", e)))
                //load selected bot
                .and_then(|mut plugin_registry| plugin_registry.load_plugin(&bot)
                    //run bot
                    .and_then(|plugin| plugin.run(secret, request))
                    //convert output to string
                    .and_then(|res| Ok(res.to_string()))
                )
            ) {
            Ok(out) => Response::new().with_status(StatusCode::Ok).with_body(out),
            Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(e),
        }
    }
}

/// Hyper Service implementation
impl Service for WebServer {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match req.method() {
            &Method::Post => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^/Telegram/([^/]+)/([^/]+)").expect("Unable to compile regexp");
                }

                //extract bot and secret from URL
                let path = String::from(req.path());
                match RE.captures(&path) {
                    Some(matches) => {
                        let bot = String::from(&matches[1]);
                        let secret = String::from(&matches[2]);
                        //concat every request's body chunk
                        Box::new(req.body().concat2().map(move |chunks| { WebServer::map_body(bot, secret, chunks.to_vec()) }))
                    },
                    None => Box::new(futures::future::ok(
                        Response::new().with_status(StatusCode::NotFound)
                    ))
                }
            },
            _ => {
                Box::new(futures::future::ok(
                    Response::new().with_body("Rustegram server")
                ))
            },
        }
    }
}
