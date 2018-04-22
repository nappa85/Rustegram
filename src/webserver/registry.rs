extern crate dynamic_reload;
extern crate notify;
extern crate serde_json;
extern crate toml;
extern crate client_lib;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;

use self::dynamic_reload::{DynamicReload, Search, Lib, UpdateState, Symbol, PlatformName};

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use self::serde_json::value::Value as JsonValue;

use self::toml::Value as TomlValue;

use self::client_lib::entities::Request;
use self::client_lib::session::Session;

pub struct Plugin {
    name: String,
    config: Arc<RwLock<TomlValue>>,
    session: Arc<RwLock<Session>>,
    plugins: Vec<(Arc<Lib>, Arc<Symbol<'static, extern "C" fn(config: *const Arc<RwLock<TomlValue>>, session: *const Arc<RwLock<Session>>, secret: &str, request: *const &Request) -> *const Result<JsonValue, String>>>)>,
}

impl Plugin {
    pub fn new(name: &str) -> Result<Plugin, String> {
        Ok(Plugin {
            name: name.to_owned(),
            config: Arc::new(RwLock::new(Plugin::load_config(name)?)),
            session: Arc::new(RwLock::new(Session::new())),
            plugins: Vec::new()
        })
    }

    fn add_plugin(&mut self, plugin: &Arc<Lib>) {
        match unsafe { plugin.lib.get(b"init_bot\0") } {
            Ok(temp) => {
                let f: Symbol<extern "C" fn(config: *const Arc<RwLock<TomlValue>>, session: *const Arc<RwLock<Session>>, secret: &str, request: *const &Request) -> *const Result<JsonValue, String>> = temp;
                self.plugins.push((plugin.clone(), Arc::new(unsafe { transmute(f) })));
            },
            Err(e) => println!("Failed to load symbol for {}: {:?}", self.name, e),
        }
    }

    fn unload_plugins(&mut self, lib: &Arc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i].0 == lib {
                self.plugins.swap_remove(i);
            }
        }
    }

    fn reload_plugin(&mut self, lib: &Arc<Lib>) {
        Self::add_plugin(self, lib);
    }

    // called when a lib needs to be reloaded.
    fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => Self::unload_plugins(self, lib.unwrap()),
            UpdateState::After => Self::reload_plugin(self, lib.unwrap()),
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }

    pub fn run(&self, secret: String, request: &Request) -> Result<&JsonValue, String> {
        if self.plugins.len() == 0 {
            return Err(format!("Lib {} not loaded", self.name));
        }

        let f = &self.plugins[0].1;
        let res = f(Box::into_raw(Box::new(self.config.clone())), Box::into_raw(Box::new(self.session.clone())), &secret, Box::into_raw(Box::new(request)));

        unsafe {
            if res.is_null() {
                Err(format!("Null pointer exception"))
            }
            else {
                match *res {
                    Ok(ref v) => Ok(v),
                    Err(ref e) => Err(e.to_string()),
                }
            }
        }
    }

    fn set_config(&self, lib: &str) -> Result<(), String> {
        match self.config.write() {
            Ok(mut config) => {
                *config = Plugin::load_config(lib)?;
                println!("Reloaded config for {}", lib);
                Ok(())
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }

    fn load_config(lib: &str) -> Result<TomlValue, String> {
        let mut config_file = PathBuf::new();
        config_file.push("config");
        config_file.push(lib);
        config_file.set_extension("toml");
        (File::open(&config_file).map_err(|e| format!("File {:?} not found: {:?}", config_file, e)))
            .and_then(|mut toml| {
                let mut s = String::new();
                (toml.read_to_string(&mut s).map_err(|e| format!("Unable to read Toml file {:?}: {:?}", config_file, e)))
                    .and_then(|_| toml::from_str(&s).map_err(|e| format!("Syntax error on Toml file {:?}: {:?}", config_file, e)))
            })
    }
}

pub struct PluginRegistry {
    handler: DynamicReload<'static>,
    libs: HashMap<String, Plugin>,
    _watcher: RecommendedWatcher,
    watch_recv: Receiver<DebouncedEvent>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        let (tx, rx) = channel();

        //those expect are fine, if it fails we want to panic!
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).expect("Unable to init config watcher");
        watcher.watch(Path::new("config"), RecursiveMode::NonRecursive).expect("Unable to watch config dir");

        // Setup the reload handler. A temporary directory will be created inside the tmp folder
        // where plugins will be loaded from. That is because on some OS:es loading a shared lib
        // will lock the file so we can't overwrite it so this works around that issue.
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: HashMap::new(),
            _watcher: watcher,
            watch_recv: rx,
        }
    }

    pub fn load_plugin(&mut self, lib: &str) -> Result<&mut Plugin, String> {
        loop {
            match self.watch_recv.try_recv() {
                Ok(event) => match event {
                    DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => match path.as_path().file_stem() {
                        Some(stem) => {
                            match stem.to_str() {
                                Some(filename) => if self.libs.contains_key(filename) {
                                    match self.libs.get(filename) {
                                        Some(plugin) => plugin.set_config(filename)?,
                                        None => { return Err(format!("Plugin disappeared: {}", filename)); },
                                    }
                                },
                                None => {},
                            }
                        },
                        None => {},
                    },
                    _ => {},
                },
                Err(e) => if e == TryRecvError::Empty { break; } else { println!("watch error: {:?}", e) },
            }
        }

        if self.libs.contains_key(lib) {
            match self.libs.get_mut(lib) {
                Some(mut plugin) => {
                    self.handler.update(Plugin::reload_callback, &mut plugin);
                    return Ok(plugin);
                },
                None => { return Err(format!("Plugin disappeared: {}", lib)); },
            }
        }

        match self.handler.add_library(lib, PlatformName::Yes) {
            Ok(plug) => {
                self.libs.insert(lib.to_owned(), Plugin::new(lib)?);
                match self.libs.get_mut(lib) {
                    Some(mut plugin) => {
                        plugin.add_plugin(&plug);
                        self.handler.update(Plugin::reload_callback, &mut plugin);
                        Ok(plugin)
                    },
                    None => Err(format!("Plugin disappeared: {}", lib)),
                }
            },
            Err(e) => Err(format!("Error loading plugin for {}: {}", lib, e)),
        }
    }
}
