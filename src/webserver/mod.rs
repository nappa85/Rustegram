extern crate hyper;
extern crate futures;
extern crate regex;

use std::sync::Arc;

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::regex::Regex;

// use std::path::Path;
// use std::process::Command;

mod registry;

// lazy_static! {
    static plugin_registry: Arc<registry::PluginRegistry<'static>> = Arc::new(registry::PluginRegistry::new());
// }

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
        match req.method() {
            &Method::Post => {
                let path = req.path().to_owned();

                //concat every request's body chunk
                Box::new(req.body().concat2().map(move |chunks| {
                    //convert chunks to String
                    let body = String::from_utf8(chunks.to_vec()).unwrap();

                    lazy_static! {
                        static ref RE: Regex = Regex::new(r"^/Telegram/([^/]+)/([^/]+)").unwrap();
                    }

                    match RE.captures(&path) {
                        Some(matches) => {
                            //load bot library
                            //this improves modularity
                            match plugin_registry.load_plugin(&matches[1]) {
                                Ok(()) => {
                                        match plugin_registry.run(&matches[1], &matches[2], body) {
                                            Ok(out) => Response::new().with_status(StatusCode::Ok).with_body(out),
                                            Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(e),
                                        }
                                    },
                                Err(e) => Response::new().with_status(StatusCode::InternalServerError).with_body(e),
                            }
//                             //it doesn't seem to exists an "is_executable" method
//                             if Path::new(&path).exists() {
//                                 let output = Command::new(path).arg(&matches[2]).arg(body).output().unwrap();
//                                 if output.status.success() {
//                                     Response::new()
//                                         .with_status(StatusCode::Ok)
//                                         .with_body(output.stdout)
//                                 }
//                                 else {
//                                     Response::new()
//                                         .with_status(StatusCode::InternalServerError)
//                                         .with_body(output.stderr)
//                                 }
//                             }
//                             else {
//                                 Response::new().with_status(StatusCode::NotFound)
//                             }
                        },
                        None => {
                            Response::new().with_status(StatusCode::NotFound)
                        }
                    }
                }))
            },
            _ => {
                Box::new(futures::future::ok(
                    Response::new().with_body("Rustegram server")
                ))
            },
        }
    }
}
