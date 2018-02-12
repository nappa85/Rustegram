extern crate dynamic_reload;

use std::collections::HashMap;
use std::sync::Arc;

use self::dynamic_reload::{DynamicReload, Error, Search, Lib, UpdateState, Symbol, PlatformName};

struct Plugin {
    callable: bool,
    lib: Arc<Lib>,
//     fun: Symbol<'a, extern "C" fn(secret: &str, body: String) -> Result<String, String>>,
}

impl Plugin {
    pub fn new(plug: &Arc<Lib>) -> Plugin {
        let temp = plug.clone();
        Plugin {
            callable: true,
            lib: temp,
//             fun: unsafe {
//                 temp.lib.get(b"init_bot\0").unwrap()
//             }
        }
    }

    pub fn reload_callback(&mut self, state: UpdateState, plug: Option<&Arc<Lib>>) {
        match state {
            UpdateState::Before => {
                self.callable = false;
            },
            UpdateState::After => {
                self.lib = plug.unwrap().clone();
//                 self.fun = unsafe {
//                     self.lib.lib.get(b"init_bot\0").unwrap()
//                 };
                self.callable = true;
            },
            UpdateState::ReloadFailed(_) => println!("Failed to reload"),
        }
    }

    pub fn run(&self, secret: &str, body: String) -> Result<String, String> {
//         let f = self.fun;
        // In a real program you want to cache the symbol and not do it every time if your
        // application is performance critical
        let f: Symbol<extern "C" fn(secret: &str, body: String) -> Result<String, String>> = unsafe {
            self.lib.lib.get(b"init_bot\0").unwrap()
        };
        f(secret, body)
    }
}

pub struct PluginRegistry {
    handler: DynamicReload<'static>,
    libs: HashMap<&'static str, Plugin>,
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, lib: &'static str) -> Result<(), Error> {
        if !self.libs.contains_key(lib) {
            let plug = self.handler.add_library(lib, PlatformName::Yes)?;
            self.libs.insert(lib, Plugin::new(&plug));
        }
        else {
            let mut plugin = self.libs.get_mut(lib).unwrap();
            self.handler.update(Plugin::reload_callback, &mut plugin);
        }

        Ok(())
    }

    pub fn run(&self, lib: &str, secret: &str, body: String) -> Result<String, String> {
        let plugin = self.libs.get(lib).unwrap();
        plugin.run(secret, body)
    }
}
