extern crate hyper;

#[macro_use]
extern crate lazy_static;

mod webserver;

use hyper::server::Http;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use webserver::WebServer as WebServer;

//singleton
lazy_static! {
    static ref SESSION: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
}

fn main() {
	//start webserver
	let addr = "127.0.0.1:3000".parse().unwrap();
	let server = Http::new().bind(&addr, || Ok(WebServer)).unwrap();
	server.run().unwrap();
}
