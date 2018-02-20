# Rustegram
Telegram bot library written in Rust

The project is composed of a webserver (hyper), supporting both HTTP and HTTPS, a base lybrary for implementig bots and the single bots implementations.<br/>
Single bots are libraries too, compiled as dylibs, included by webserver only when needed.<br/>
The webserver infere the name of the dylib from URL, also asking for a secret to enforce security.<br/>
Every dylib has a dedicated toml config file.
