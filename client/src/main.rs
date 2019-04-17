#![warn(rust_2018_idioms)]

use std::env;
use lib::{self, Symbol, Library};


type LoadFunc = unsafe extern fn() -> ();

#[derive(Debug, Default)]
struct PluginManager {
    plugins: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(&mut self, paths: &[String]) {
        for path in paths {
            match Library::new(path) {
                Ok(l) => self.plugins.push(l),
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        }
    }

    pub fn handle_on_load(&self) {
        for plugin in &self.plugins {
            unsafe {
                match plugin.get::<Symbol<'_, LoadFunc>>(b"on_load") {
                    Ok(fun) => fun(),
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}


fn main() {
    let libs: Vec<_> = env::args().skip(1).collect();

    if libs.is_empty() {
        println!("No plugins specified");
        std::process::exit(1);
    }

    let mut plugman = PluginManager::new();
    plugman.init(&libs);
    plugman.handle_on_load();
}
