#![warn(rust_2018_idioms)]

#[macro_use]
mod macros;

pub mod plugin;
pub mod plugin_manager;


pub use self::{
    plugin::Plugin,
    plugin_manager::PluginManager,
};
