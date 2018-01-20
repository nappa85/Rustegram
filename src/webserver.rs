extern crate hyper;
extern crate futures;
extern crate serde_json;
extern crate regex;
// extern crate ini;

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::serde_json::Value;

use self::regex::Regex;

pub struct WebServer;

impl Service for WebServer {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match req.method() {//, req.path()) {
            &Method::Post => {
                let path = format!("{}", req.path());

                //concat every request's body chunk
                Box::new(req.body().concat2().map(move |chunks| {
                    //convert chunks to String
                    let body = String::from_utf8(chunks.to_vec()).unwrap();

                    let mut res = Response::new();

                    // Parse the string of data into serde_json::Value.
                    let json:Value = match serde_json::from_str(&body){
                        Ok(val) => val,
                        Err(err) => {
                            res.set_status(StatusCode::InternalServerError);
                            res.set_body(format!("{}", err));
                            Value::Null
                        }
                    };

                    if json != Value::Null {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^/Telegram/([^/]+)/([^/]+)").unwrap();
                        }

                        match RE.captures(&path) {
                            Some(matches) => {
                                res.set_body(format!("class: {}\ncode: {}", &matches[1], &matches[2]));
                            },
                            None => {
                                res.set_status(StatusCode::NotFound);
                            }
                        }
                    }

                    res
                }))
            },
            _ => {
                Box::new(futures::future::ok(
                    Response::new().with_status(StatusCode::NotFound)
                ))
            },
        }
    }
}
