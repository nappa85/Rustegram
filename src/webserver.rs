extern crate hyper;
extern crate futures;
// extern crate serde_json;
extern crate regex;

use self::futures::future::Future;
use self::futures::Stream;

use self::hyper::{Method, StatusCode};
use self::hyper::server::{Request, Response, Service};

use self::regex::Regex;

// use self::serde_json::Value;

use std::path::Path;
use std::process::Command;

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

//                     // Parse the string of data into serde_json::Value.
//                     let json:Value = match serde_json::from_str(&body){
//                         Ok(val) => val,
//                         Err(err) => {
//                             res.set_status(StatusCode::InternalServerError);
//                             res.set_body(format!("{}", err));
//                             Value::Null
//                         }
//                     };
// 
//                     if json != Value::Null {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^/Telegram/([^/]+)/([^/]+)").unwrap();
                        }

                        match RE.captures(&path) {
                            Some(matches) => {
                                //delegate to another executable bot's execution
                                //this improves modularity
                                let path = format!("./bots/{}", &matches[1]);
                                //it doesn't seem to exists an "is_executable" method
                                if Path::new(&path).exists() {
                                    let output = Command::new(path).arg(&matches[2]).arg(body).output().unwrap();
                                    if output.status.success() {
                                        Response::new()
                                            .with_status(StatusCode::Ok)
                                            .with_body(output.stdout)
                                    }
                                    else {
                                        Response::new()
                                            .with_status(StatusCode::InternalServerError)
                                            .with_body(output.stderr)
                                    }
                                }
                                else {
                                    Response::new().with_status(StatusCode::NotFound)
                                }
                            },
                            None => {
                                Response::new().with_status(StatusCode::NotFound)
                            }
                        }
//                     }
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
