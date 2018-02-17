extern crate dynamic_reload;
extern crate serde_json;

use std::sync::Arc;

use self::dynamic_reload::{DynamicReload, Search, Lib, UpdateState, Symbol, PlatformName};

use self::serde_json::value::Value;

pub struct Plugin {
    name: String,
    plugins: Vec<Arc<Lib>>,
}

impl Plugin {
    pub fn new(name: String) -> Plugin {
        Plugin {
            name: name,
            plugins: Vec::new()
        }
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

    pub fn run(&self, secret: &str, body: String) -> Result<Value, String> {
        if self.plugins.len() > 0 {
            // In a real program you want to cache the symbol and not do it every time if your
            // application is performance critical
            match unsafe { self.plugins[0].lib.get(b"init_bot\0") } {
                Ok(temp) => {
                    let f: Symbol<extern "C" fn(secret: &str, body: String) -> Result<Value, String>> = temp;
                    f(secret, body)
                },
                Err(e) => Err(format!("Error getting Symbol for {}: {}", self.name, e)),
            }
        }
        else {
            Err(format!("Lib {} not loaded", self.name))
        }
    }
}

pub struct PluginRegistry {
    handler: DynamicReload<'static>,
    libs: Vec<Plugin>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        // Setup the reload handler. A temporary directory will be created inside the tmp folder
        // where plugins will be loaded from. That is because on some OS:es loading a shared lib
        // will lock the file so we can't overwrite it so this works around that issue.
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: Vec::new(),
        }
    }

    pub fn load_plugin(&mut self, lib: String) -> Result<&Plugin, String> {
        for i in 0..self.libs.len() {
            if self.libs[i].name == lib {
                match self.libs.get_mut(i) {
                    Some(mut plugin) => {
                        self.handler.update(Plugin::reload_callback, &mut plugin);
                        return Ok(plugin);
                    },
                    None => { return Err(format!("Plugin disappeared: {}", lib)); },
                }
            }
        }

        match self.handler.add_library(&lib, PlatformName::Yes) {
            Ok(plug) => {
                let i = self.libs.len();
                self.libs.push(Plugin::new(lib.clone()));
                match self.libs.get_mut(i) {
                    Some(mut plugin) => {
                        plugin.add_plugin(&plug);
                        self.handler.update(Plugin::reload_callback, &mut plugin);
                        return Ok(plugin);
                    },
                    None => { return Err(format!("Plugin disappeared: {}", lib)); },
                }
            },
            Err(e) => Err(format!("Error loading plugin for {}: {}", lib, e)),
        }
    }
}
