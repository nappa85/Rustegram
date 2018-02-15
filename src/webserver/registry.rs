extern crate dynamic_reload;
extern crate serde_json;

use std::collections::HashMap;
use std::sync::Arc;

use self::dynamic_reload::{DynamicReload, Search, Lib, UpdateState, Symbol, PlatformName};

use self::serde_json::value::Value;

struct Plugin {
    name: String,
    callable: bool,
    lib: Arc<Lib>,
//     fun: Symbol<'a, extern "C" fn(secret: &str, body: String) -> Result<Value, String>>,
}

impl Plugin {
    pub fn new(name: String, plug: &Arc<Lib>) -> Plugin {
        let temp = plug.clone();
        Plugin {
            name: name,
            callable: true,
            lib: temp,
//             fun: unsafe {
//                 temp.lib.get(b"init_bot\0").expect(&format!("Error getting Symbol for {}", name))
//             }
        }
    }

    pub fn reload_callback(&mut self, state: UpdateState, plug: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => {
                println!("Started Symbol update for {}", self.name);
                self.callable = false;
            },
            UpdateState::After => {
                match plug {
                    Some(temp) => {
                        println!("Symbol updated for {}", self.name);
                        self.lib = temp.clone();
                    },
                    None => {
                        println!("Symbol updated to None for {}", self.name);
                    },
                }
//                 self.fun = unsafe {
//                     self.lib.lib.get(b"init_bot\0").expect(&format!("Error updating Symbol for {}", name))
//                 };
                self.callable = true;
            },
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }

    pub fn run(&self, secret: &str, body: String) -> Result<Value, String> {
//         let f = self.fun;
        // In a real program you want to cache the symbol and not do it every time if your
        // application is performance critical
//         let f: Symbol<extern "C" fn(secret: &str, body: String) -> Result<Value, String>> = unsafe {
//             self.lib.lib.get(b"init_bot\0").expect(&format!("Error getting Symbol for {}", self.name))
//         };
//         f(secret, body)
        match unsafe { self.lib.lib.get(b"init_bot\0") } {
            Ok(temp) => {
                let f: Symbol<extern "C" fn(secret: &str, body: String) -> Result<Value, String>> = temp;
                f(secret, body)
            },
            Err(e) => Err(format!("Error getting Symbol for {}: {}", self.name, e)),
        }
    }
}

pub struct PluginRegistry {
    handler: DynamicReload<'static>,
    libs: HashMap<String, Plugin>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, lib: String) -> Result<(), String> {
        if !self.libs.contains_key(&lib) {
            match self.handler.add_library(&lib, PlatformName::Yes) {
                Ok(plug) => { self.libs.insert(lib.clone(), Plugin::new(lib, &plug)); },
                Err(e) => { return Err(format!("Error loading plugin for {}: {}", lib, e)); },
            }
        }
        else {
            match self.libs.get_mut(&lib) {
                Some(mut plugin) => { self.handler.update(Plugin::reload_callback, &mut plugin); },
                None => { return Err(format!("Error retrieving plugin for {}", lib)); },
            }
        }

        Ok(())
    }

    pub fn run(&self, lib: String, secret: String, body: String) -> Result<Value, String> {
        match self.libs.get(&lib) {
            Some(plugin) => plugin.run(&secret, body),
            None => Err(format!("Error retrieving plugin for {}", lib)),
        }
    }
}
