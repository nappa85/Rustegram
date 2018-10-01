extern crate http;
extern crate regex;

mod registry;

use serde_json;

use futures::future::{IntoFuture, Future};
use futures::Stream;

use hyper::{Method, StatusCode, Request, Response, Body};
use hyper::service::Service;

use client_lib::entities::Request as TelegramRequest;

use self::http::Error as HttpError;

use self::regex::Regex;

use self::registry::PluginRegistry;

pub struct WebServer;

/// Hyper Service implementation
impl Service for WebServer {
    // boilerplate hooking up hyper's server types
    type ReqBody = Body;
    type ResBody = Body;
    type Error = HttpError;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Response<Self::ResBody>, Error=Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let mut res = Response::builder();
        let res = match req.method() {
            &Method::POST => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"^/Telegram/([^/]+)/([^/]+)").expect("Unable to compile regexp");
                }

                //extract bot and secret from URL
                let path = String::from(req.uri().path());
                match RE.captures(&path) {
                    Some(matches) => {
                        let bot = String::from(&matches[1]);
                        let secret = String::from(&matches[2]);
                        //concat every request's body chunk
                        return Box::new(req.into_body().concat2()
                            .map_err(|e| format!("{:?}", e))
                            .and_then(move |chunks| serde_json::from_slice::<TelegramRequest>(&chunks).map_err(|e| format!("Syntax error in JSON input: {:?}", e)))
                            .and_then(move |ref request| {
                                PluginRegistry::run_plugin(&bot, secret, request)
                                    //convert output to string
                                    .and_then(|res| Ok(res.to_string()))
                            })
                            .then(move |r| match r {
                                Ok(out) => res.status(StatusCode::OK).body(Body::from(out)),
                                Err(e) => res.status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::from(e)),
                            }));
                    },
                    None => res.status(StatusCode::NOT_FOUND).body(Body::empty()),
                }
            },
            _ => res.body(Body::from("Rustegram server")),
        };
        Box::new(res.into_future())
    }
}
