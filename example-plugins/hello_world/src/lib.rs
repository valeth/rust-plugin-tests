#![warn(rust_2018_idioms)]
#![warn(clippy::all)]


use log::debug;

use plugin::{Plugin, define_plugin};


#[derive(Default)]
pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn name(&self) -> &'static str {
        "hello_world"
    }

    fn on_load(&self) {
        debug!("Hello World");
    }
}


define_plugin!(HelloWorldPlugin, Default::default);
