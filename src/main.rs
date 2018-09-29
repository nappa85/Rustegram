#![deny(warnings)]
#![deny(missing_docs)]

//! # Rustegram
//!
//! Telegram bot server with dynamically loaded bots
//!
//! Provides a C ABI to call on dynamically linked libs, which are
//! dynamicaly reloaded on file change,
//! allowing centralized configuration with automatical file reload
//! and in-memory per-bot private session.

#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate toml;
extern crate futures;
extern crate hyper;
#[cfg(feature = "https")]
extern crate rustls;
#[cfg(feature = "https")]
extern crate tokio_tcp;
#[cfg(feature = "https")]
extern crate tokio_rustls;
#[macro_use] extern crate log;
extern crate env_logger;

mod webserver;

use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::net::SocketAddr;
#[cfg(feature = "https")]
use std::sync::Arc;

use futures::{future, Future};

use hyper::Server;
use hyper::rt::run;

#[cfg(feature = "https")]
use rustls::ServerConfig;
#[cfg(feature = "https")]
use rustls::internal::pemfile;

#[cfg(feature = "https")]
use tokio_rustls::ServerConfigExt;

#[cfg(feature = "https")]
use tokio_tcp::TcpListener;

use webserver::WebServer;

#[derive(Deserialize)]
struct Config {
    address: String,
    port: Option<u64>,
    https: Option<HttpsConfig>,
}

impl Config {
    fn get_address(&self) -> Result<SocketAddr, io::Error> {
        format!("{}:{}", self.address, self.port.unwrap_or_else(|| if self.https_enabled() { 443 } else { 80 }))
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::AddrNotAvailable, e))
    }

    fn https_enabled(&self) -> bool {
        match self.https {
            Some(ref c) => c.enabled,
            None => false,
        }
    }
}

#[derive(Deserialize)]
struct HttpsConfig {
    enabled: bool,
    #[cfg(feature = "https")]
    certs: String,
    #[cfg(feature = "https")]
    private_key: String,
}

#[cfg(feature = "https")]
fn load_certs(filename: &str) -> Result<Vec<rustls::Certificate>, io::Error> {
    let certfile = File::open(filename)?;
    let mut reader = io::BufReader::new(certfile);
    pemfile::certs(&mut reader).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Error reading certs file"))
}

#[cfg(feature = "https")]
fn load_private_key(filename: &str) -> Result<rustls::PrivateKey, io::Error> {
    let keyfile = File::open(filename)?;
    let mut reader = io::BufReader::new(keyfile);
    let keys = pemfile::rsa_private_keys(&mut reader).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Error reading private_key file"))?;
    if keys.len() == 1 {
        Ok(keys[0].clone())
    }
    else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Multiple keys found"))
    }
}

#[cfg(feature = "https")]
fn serve_https(config: Config) -> Result<Box<Future<Item=(), Error=()> + Send>, io::Error> {
    let addr = config.get_address()?;

    let tls_cfg = {
        let certs = load_certs(config.https.and_then(|c| c.certs).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing certs"))?)?;
        let key = load_private_key(config.https.and_then(|c| c.private_key).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing private_key"))?)?;
        let mut cfg = ServerConfig::new();
        cfg.set_single_cert(certs, key);
        Arc::new(cfg)
    };
    let tcp = TcpListener::bind(&addr)?;

    Ok(Box::new(future::lazy(move || {
        let tls = tcp.incoming().and_then(move |s| tls_cfg.accept_async(s));
        let server = Server::builder(tls)
                            .serve(|| -> Result<WebServer, hyper::Error> { Ok(WebServer) })
                            .map_err(|e| {
                                error!("server error: {}", e);
                            });

        info!("Listening on https://{}", addr);

        server
    })))
}

#[cfg(not(feature = "https"))]
fn serve_https(_: Config) -> Result<Box<Future<Item=(), Error=()> + Send>, io::Error> {
    Err(io::Error::new(io::ErrorKind::PermissionDenied, "Rustegran compiled without https support"))
}

fn serve(config: Config) -> Result<Box<Future<Item=(), Error=()> + Send>, io::Error> {
    let addr = config.get_address()?;

    Ok(Box::new(future::lazy(move || {
        let server = Server::bind(&addr)
                            .serve(|| -> Result<WebServer, hyper::Error> { Ok(WebServer) })
                            .map_err(|e| {
                                error!("server error: {}", e);
                            });

        info!("Listening on http://{}", addr);

        server
    })))
}

/// Launch WebServer according to config
fn main() {
    env_logger::init();

    info!("starting up");

    let args:Vec<String> = env::args().collect();

    // config file can be the first argument
    let config_file = if args.len() > 1 {
        args.get(1).expect("Cannot retrieve config path").to_owned()
    }
    else {
        let path = Path::new(args.get(0).expect("Cannot find executable path"));
        format!("config/{}.toml", path.file_stem().expect("Cannot find executable name").to_str().expect("Cannot parse executable name"))
    };
    let mut toml = File::open(&config_file).expect(&format!("File {} not found", config_file));
    let mut s = String::new();
    toml.read_to_string(&mut s).expect("Unable to read Toml file");
    //read config file in toml format
    let config: Config = toml::from_str(&s).expect("Syntax error on Tolm file");

    run(if config.https_enabled() { serve_https(config) } else { serve(config) }.expect("Unable to start WebServer"));
}
