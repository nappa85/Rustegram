extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;
extern crate ini;

use std::collections::HashMap;
use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::{Client, Method};
use hyper::client::Request;
use tokio_core::reactor::Core;
use url::form_urlencoded;
use ini::Ini;

pub struct Telegram {
    config: Ini,
}

impl Telegram {
    fn callTelegram(&self, method: &str, params: HashMap) {
        let mut core = Core::new()?;
        let client = Client::new(&core.handle());

        let uri = ("https://api.telegram.org/bot".to_owned() + self.config.get("HTTP_TOKEN")? + '/' + method).parse()?;
        let mut req = Request::new(Method::Post, uri);
        let body = form_urlencoded::byte_serialize(params.into_iter());
        req.set_body(body);
        let work = client.request(req).and_then(|res| {
            res.body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(From::from)
            })
        });
        core.run(work)?;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
