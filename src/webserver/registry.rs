extern crate dynamic_reload;
extern crate notify;
extern crate serde_json;
extern crate toml;

use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::fs::File;
use std::io::Read;

use self::dynamic_reload::{DynamicReload, Search, Lib, UpdateState, Symbol, PlatformName};

use self::notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use self::serde_json::value::Value as JsonValue;

use self::toml::Value as TomlValue;

pub struct Plugin {
    name: String,
    config: TomlValue,
    plugins: Vec<Arc<Lib>>,
}

impl Plugin {
    pub fn new(name: String) -> Result<Plugin, String> {
        Ok(Plugin {
            name: name.clone(),
            config: Plugin::load_config(name)?,
            plugins: Vec::new()
        })
    }

    fn add_plugin(&mut self, plugin: &Arc<Lib>) {
        self.plugins.push(plugin.clone());
    }

    fn unload_plugins(&mut self, lib: &Arc<Lib>) {
        for i in (0..self.plugins.len()).rev() {
            if &self.plugins[i] == lib {
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

    pub fn run(&self, secret: String, body: JsonValue) -> Result<JsonValue, String> {
        if self.plugins.len() > 0 {
            // In a real program you want to cache the symbol and not do it every time if your
            // application is performance critical
            match unsafe { self.plugins[0].lib.get(b"init_bot\0") } {
                Ok(temp) => {
                    let f: Symbol<extern "C" fn(config: *const TomlValue, secret: &str, body: *const JsonValue) -> Result<JsonValue, String>> = temp;
                    f(Box::into_raw(Box::new(self.config.clone())), &secret, Box::into_raw(Box::new(body)))
                },
                Err(e) => Err(format!("Error getting Symbol for {}: {}", self.name, e)),
            }
        }
        else {
            Err(format!("Lib {} not loaded", self.name))
        }
    }

    fn load_config(lib: String) -> Result<TomlValue, String> {
        let config_file = format!("config/{}.toml", lib);
        match File::open(&config_file) {
            Ok(mut toml) => {
                let mut s = String::new();
                match toml.read_to_string(&mut s) {
                    Ok(_) => match toml::from_str(&s) {
                        Ok(config) => Ok(config),
                        Err(e) => Err(format!("Syntax error on Toml file {}: {}", config_file, e)),
                    },
                    Err(e) => Err(format!("Unable to read Toml file {}: {}", config_file, e)),
                }
            },
            Err(e) => Err(format!("File {} not found: {}", config_file, e)),
        }
    }
}

pub struct PluginRegistry {
    handler: DynamicReload<'static>,
    libs: HashMap<String, Plugin>,
//     watcher: RecommendedWatcher,
    watch_recv: Receiver<DebouncedEvent>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        let (tx, rx) = channel();

        //those expect are fine, if it fails we want to panic!
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).expect("Unable to init config watcher");
        watcher.watch(Path::new("config/"), RecursiveMode::NonRecursive).expect("Unable to watch config dir");

        // Setup the reload handler. A temporary directory will be created inside the tmp folder
        // where plugins will be loaded from. That is because on some OS:es loading a shared lib
        // will lock the file so we can't overwrite it so this works around that issue.
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: HashMap::new(),
//             watcher: watcher,
            watch_recv: rx,
        }
    }

    pub fn load_plugin(&mut self, lib: String) -> Result<&Plugin, String> {
        loop {
            match self.watch_recv.try_recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => if e == TryRecvError::Empty { break; } else { println!("watch error: {:?}", e) },
            }
        }

        if self.libs.contains_key(&lib) {
            match self.libs.get_mut(&lib) {
                Some(mut plugin) => {
                    self.handler.update(Plugin::reload_callback, &mut plugin);
                    return Ok(plugin);
                },
                None => { return Err(format!("Plugin disappeared: {}", lib)); },
            }
        }

        match self.handler.add_library(&lib, PlatformName::Yes) {
            Ok(plug) => {
                self.libs.insert(lib.clone(), Plugin::new(lib.clone())?);
                match self.libs.get_mut(&lib) {
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
