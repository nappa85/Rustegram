extern crate dynamic_reload;
extern crate notify;
extern crate parking_lot;

use std::thread::{spawn, JoinHandle};
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;

use serde_json::value::Value as JsonValue;

use toml::{self, Value as TomlValue};

use client_lib::entities::Request;
use client_lib::session::Session;

use self::dynamic_reload::{DynamicReload, Search, Lib, UpdateState, Symbol, PlatformName};

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use self::parking_lot::{RwLock, Mutex};

//singleton
lazy_static! {
    static ref REGISTRY: Arc<RwLock<PluginRegistry>> = Arc::new(RwLock::new(PluginRegistry::new()));
}

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
            Err(e) => error!("Failed to load symbol for {}: {:?}", self.name, e),
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
            UpdateState::ReloadFailed(_) => error!("Failed to reload"),
        }
    }

    pub fn run(&self, secret: String, request: &Request) -> Result<JsonValue, String> {
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
                    Ok(ref v) => Ok(v.clone()),
                    Err(ref e) => Err(e.clone()),
                }
            }
        }
    }

    fn set_config(&self, lib: &str) -> Result<(), String> {
        let mut config = self.config.write();
        *config = Plugin::load_config(lib)?;
        info!("Reloaded config for {}", lib);
        Ok(())
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
    handler: Mutex<DynamicReload<'static>>,
    libs: RwLock<HashMap<String, RwLock<Plugin>>>,
    _config_thread: JoinHandle<()>,
}

impl PluginRegistry {
    fn new() -> PluginRegistry {
        // Setup the reload handler. A temporary directory will be created inside the tmp folder
        // where plugins will be loaded from. That is because on some OS:es loading a shared lib
        // will lock the file so we can't overwrite it so this works around that issue.
        PluginRegistry {
            handler: Mutex::new(DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default)),
            libs: RwLock::new(HashMap::new()),
            _config_thread: spawn(move || {
                let (tx, rx) = channel();

                //those expect are fine, if it fails we want to panic!
                let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).expect("Unable to init config watcher");
                watcher.watch(Path::new("config"), RecursiveMode::NonRecursive).expect("Unable to watch config dir");

                loop {
                    match rx.recv() {
                        Ok(event) => match event {
                            DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => match path.as_path().file_stem().and_then(|stem| stem.to_str()) {
                                Some(filename) => {
                                    let reg = REGISTRY.clone();
                                    let singleton = reg.read();
                                    let libs = singleton.libs.read();
                                    if libs.contains_key(filename) {
                                        match libs.get(filename)
                                                .ok_or_else(|| format!("Plugin disappeared: {}", filename))
                                                .and_then(|plugin| plugin.write().set_config(filename)) {
                                            Err(e) => error!("{}", e),
                                            _ => {},
                                        }
                                    }
                                },
                                None => {},
                            },
                            _ => {},
                        },
                        Err(e) => error!("watch error: {:?}", e),
                    }
                }
            }),
        }
    }

    pub fn run_plugin<'a>(lib: &str, secret: String, request: &Request) -> Result<JsonValue, String> {
        let reg = REGISTRY.clone();
        let pr = reg.read();
        pr._run_plugin(lib, secret, request)
    }

    fn _run_plugin(&self, lib: &str, secret: String, request: &Request) -> Result<JsonValue, String> {
        {
            let libs = self.libs.read();
            if libs.contains_key(lib) {
                match libs.get(lib) {
                    Some(plugin) => {
                        self.handler.lock().update(Plugin::reload_callback, &mut plugin.write());
                        return plugin.read().run(secret, request);
                    },
                    None => { return Err(format!("Plugin disappeared: {}", lib)); },
                }
            }
        }

        match self.handler.lock().add_library(lib, PlatformName::Yes) {
            Ok(plug) => {
                {
                    self.libs.write().insert(lib.to_owned(), RwLock::new(Plugin::new(lib)?));
                }
                let libs = self.libs.read();
                match libs.get(lib) {
                    Some(plugin) => {
                        {
                            let mut wplugin = plugin.write();
                            wplugin.add_plugin(&plug);
                            self.handler.lock().update(Plugin::reload_callback, &mut wplugin);
                        }
                        plugin.read().run(secret, request)
                    },
                    None => Err(format!("Plugin disappeared: {}", lib)),
                }
            },
            Err(e) => Err(format!("Error loading plugin for {}: {}", lib, e)),
        }
    }
}
