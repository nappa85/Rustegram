#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_proto;

mod webserver;

use std::env;
use std::fs::File;
use std::io::Read;

use hyper::server::Http;
use native_tls::{TlsAcceptor, Pkcs12};
use tokio_proto::TcpServer;
use tokio_tls::proto;

use webserver::WebServer as WebServer;

fn main() {
    let args:Vec<String> = env::args().collect();
    assert!(args.len() == 2 || args.len() == 4, format!("Usage: {} <IP:PORT> [<p12 file> <password>]", args.get(0).unwrap()));

    if args.len() == 4 {
        // Create our TLS context through which new connections will be
        // accepted. This is where we pass in the certificate as well to
        // send to clients.
        let mut f = File::open(args.get(2).unwrap()).expect("file not found");
        let mut der = Vec::new();
        f.read_to_end(&mut der).unwrap();
        let cert = Pkcs12::from_der(der.as_slice(), args.get(3).unwrap()).unwrap();
        let tls_cx = TlsAcceptor::builder(cert).unwrap().build().unwrap();

        // Wrap up hyper's `Http` protocol in our own `Server` protocol. This
        // will run hyper's protocol and then wrap the result in a TLS stream,
        // performing a TLS handshake with connected clients.
        let proto = proto::Server::new(Http::new(), tls_cx);

        // Finally use `tokio-proto`'s `TcpServer` helper struct to quickly
        // take our protocol above to running our hello-world Service on a
        // local TCP port.
        let addr = args.get(1).unwrap().parse().unwrap();
        let srv = TcpServer::new(proto, addr);
        srv.serve(|| Ok(WebServer));
    }
    else {
        //start normal webserver
        let addr = args.get(1).unwrap().parse().unwrap();
        let server = Http::new().bind(&addr, || Ok(WebServer)).unwrap();
        server.run().unwrap();
    }
}
