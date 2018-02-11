extern crate dynamic_reload;

use std::collections::HashMap;
use std::sync::Arc;

use self::dynamic_reload::{DynamicReload, Error, Search, Lib, UpdateState, Symbol, PlatformName};

pub struct PluginRegistry<'a> {
    handler: DynamicReload<'a>,
    libs: HashMap<&'a str, Symbol<'a, extern "C" fn(secret: &str, body: String) -> Result<String, String>>>,//TODO
}

impl<'a> PluginRegistry<'a> {
    pub fn new() -> PluginRegistry<'static> {
        PluginRegistry {
            handler: DynamicReload::new(Some(vec!["bots"]), Some("tmp"), Search::Default),
            libs: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, lib: &str) -> Result<(), Error> {
        if !self.libs.contains_key(lib) {
            match self.handler.add_library(lib, PlatformName::Yes) {
                Ok(plugin) => self.libs.insert(lib, plugin.get(b"shared_fun\0").unwrap()),
                Err(e) => Err(e),
            }
        }
        else {
            self.handler.update(Self::reload_callback, self);
        }

        Ok(())
    }

    pub fn reload_callback(&mut self, state: UpdateState, lib: Option<&Arc<Lib>>) {
        //do we really care?
    }

    pub fn run(&self, lib: &str, secret: &str, body: String) -> Result<String, String> {
        let fun = self.libs.get(lib).unwrap();
        fun(secret, body)
    }
}
