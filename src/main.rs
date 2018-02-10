#[macro_use]
extern crate lazy_static;
extern crate hyper;
extern crate native_tls;
extern crate tokio_tls;
extern crate tokio_proto;
extern crate toml;

mod webserver;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use hyper::server::Http;
use native_tls::{TlsAcceptor, Pkcs12};
use tokio_proto::TcpServer;
use tokio_tls::proto;
use toml::Value;

use webserver::WebServer as WebServer;

fn main() {
    let args:Vec<String> = env::args().collect();

    let config_file = if args.len() > 1 {
        args.get(1).expect("Cannot retrieve config path").to_owned()
    }
    else {
        let path = Path::new(args.get(0).expect("Cannot find executable path"));
        format!("{}.toml", path.file_stem().expect("Cannot find executable name").to_str().expect("Cannot parse executable name"))
    };
    let mut toml = File::open(&config_file).expect(&format!("File {} not found", config_file));
    let mut s = String::new();
    toml.read_to_string(&mut s).expect("Unable to read Toml file");
    let config:Value = toml::from_str(&s).expect("Syntax error on Tolm file");
    let https = config["https"]["enabled"].as_bool().expect("Error interpreting https.enabled flag");

    let addr = format!("{}:{}", config["address"].as_str().expect("Error interpreting address value"), if config.get("port").is_none() { if https { "443" } else { "80" } } else { config["port"].as_str().expect("Error interpreting port value") }).parse().expect("Error parsing webserver address");

    if https {
        // Create our TLS context through which new connections will be
        // accepted. This is where we pass in the certificate as well to
        // send to clients.
        let p12_file = config["https"]["identity"].as_str().expect("Error interpreting https.identity value");
        let mut p12 = File::open(&p12_file).expect(&format!("Identity {} not found", p12_file));
        let mut der = Vec::new();
        p12.read_to_end(&mut der).expect("Unable to read identity file");
        let cert = Pkcs12::from_der(der.as_slice(), config["https"]["password"].as_str().expect("Error interpreting https.password value")).expect("Syntax error on identity file");
        let tls_cx = TlsAcceptor::builder(cert).expect("Error on TLS init").build().expect("Error on TLS build");

        // Wrap up hyper's `Http` protocol in our own `Server` protocol. This
        // will run hyper's protocol and then wrap the result in a TLS stream,
        // performing a TLS handshake with connected clients.
        let proto = proto::Server::new(Http::new(), tls_cx);

        // Finally use `tokio-proto`'s `TcpServer` helper struct to quickly
        // take our protocol above to running our Service on a local TCP port.
        let srv = TcpServer::new(proto, addr);
        srv.serve(|| Ok(WebServer));
    }
    else {
        //start normal webserver
        let server = Http::new().bind(&addr, || Ok(WebServer)).expect("Error on webserver init");
        server.run().expect("Error on webserver run");
    }
}
