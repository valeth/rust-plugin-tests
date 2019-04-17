#![warn(rust_2018_idioms)]
#![warn(clippy::all)]


use log::debug;

use plugin::{Plugin, define_plugin};


#[derive(Default)]
pub struct ByeWorldPlugin;

impl Plugin for ByeWorldPlugin {
    fn name(&self) -> &'static str {
        "bye_world"
    }

    fn on_load(&self) {
        debug!("Bye World");
    }
}


define_plugin!(ByeWorldPlugin, Default::default);
