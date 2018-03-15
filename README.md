# Rustegram
Telegram bot library written in Rust

The project is composed of a webserver (hyper), supporting both HTTP and HTTPS, a base library for implementing bots and the single bots implementations.<br/>
Single bots are libraries too, compiled as dylibs, included by webserver only when needed.<br/>
The webserver infer the name of the dylib from URL, also asking for a secret to enforce security.<br/>
Every dylib has a dedicated toml config file.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

The main prerequisite is having Rust, and his package manager Cargo, installed.<br/>
The easiest way to obtain it through your favourite package manager, or following the [official guide](https://www.rust-lang.org/en-US/install.html).<br/>
And, obviously, you'll need [git](https://git-scm.com/) to retrieve the source code.

### Installing

First of all, checkout the source code in a folder of your preference

```
$ git clone https://github.com/nappa85/Rustegram
```

Now let's build the main module: the webserver

```
$ cd Rustegram
$ cargo build
$ nano config/rustegram.toml
```

nano is just an example, you can use your favourite editor to fill che config parameters.

For every bot you want to ship, you'll need to build it too.<br/>
Replace &lt;bot&gt; with bot's name

```
$ cd bots/<bot>
$ cargo build
$ cd ..
$ ln -sf <bot>/target/debug/lib<bot>.so
$ cp <bot>/<bot>.toml.example ../config/<bot>.toml
$ cd ..
$ nano config/<bot>.toml
```
These instructions links the file .so, if you're on OsX link the .dylib file, if you're on Windows link the .dll file.

To start the webserver simply execute
```
$ cargo run
```

Rustegram will always reply with a simple "Rustegram server" message if not called via POST.<br />
A simple test could be done using curl:
```
$ curl http://localhost:8080/Telegram/<bot>/test --data "{}"
Error during bot init: Secret mismatch
```
This should be the output, unless you've used "test" as bot's secret.

## Deployment

To deply Rustegram to production you have to build it with the optimizations

```
$ cd Rustegram
$ cargo build --release
$ cp target/release/rustegram .
```

And, again, for every bot you want to ship

```
$ cd bots/<bot>
$ cargo build --release
$ cd ..
$ cp <bot>/target/release/lib<bot>.so .
$ cp <bot>/<bot>.toml.example ../config/<bot>.toml
$ cd ..
$ nano config/<bot>.toml
```

Same note as installation step occours here: change the file extension based on your plaform.<br/>
Note that this time we used cp instead of ln, because normally on a production system you aren't going to keep the sources.<br/>
Indeed, at this point the various src/ and target/ folders are useless, along with bots subfolders.
