extern crate hyper;
extern crate futures;
extern crate regex;
extern crate serde_json;

use std::sync::{Arc, Mutex};

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::regex::Regex;

mod registry;

//singleton
lazy_static! {
    static ref REGISTRY: Arc<Mutex<registry::PluginRegistry>> = Arc::new(Mutex::new(registry::PluginRegistry::new()));
}

pub struct WebServer;

impl WebServer {
    fn map_body(bot: String, secret: String, chunks: Vec<u8>) -> Response {
        //convert chunks to String
        match String::from_utf8(chunks) {
            Ok(body) => match serde_json::from_str(&body) {
                Ok(body_value) => {
                    //load bot library
                    //this improves modularity
                    let reg = REGISTRY.clone();
                    let temp = reg.lock();
                    match temp {
                        Ok(mut plugin_registry) => match plugin_registry.load_plugin(bot.clone()) {
                            Ok(plugin) => match plugin.run(secret, body_value) {
                                Ok(out) => Response::new().with_status(StatusCode::Ok).with_body(out.to_string()),
                                Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(e),
                            },
                            Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(e),
                        },
                        Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(format!("Unable to lock plugin registry: {}", e)),
                    }
                },
                Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(format!("Syntax error on json request: {}", e)),
            },
            Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(format!("Unable to convert request body to string: {}", e)),
        }
    }
}

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

                let path = String::from(req.path());
                match RE.captures(&path) {
                    //concat every request's body chunk
                    Some(matches) => {
                        let bot = String::from(&matches[1]);
                        let secret = String::from(&matches[2]);
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
