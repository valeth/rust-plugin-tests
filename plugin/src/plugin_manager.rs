use std::path::Path;

use log::debug;
use lib::{self, Symbol, Library};

use crate::plugin::{Plugin, CreateFn};


#[derive(Default)]
pub struct PluginManager {
    loaded: Vec<Library>,
    plugins: Vec<Box<dyn Plugin>>,
    logger : Option<&'static dyn log::Log>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_logger(&mut self, logger: &'static dyn log::Log) {
        self.logger = Some(logger);
    }

    pub fn load_plugins<P>(&mut self, paths: &[P]) -> Result<(), String>
        where P: AsRef<Path>
    {
        for path in paths {
            self.load_plugin(path)?;
        }
        Ok(())
    }

    pub fn load_plugin<P>(&mut self, path: P) -> Result<(), String>
        where P: AsRef<Path>
    {
        let path = path.as_ref();
        let lib = Library::new(path.as_os_str())
            .map_err(|e| format!("Failed to load plugin {}: {}", path.display(), e))?;

        let plugin = unsafe {
            let ctor = lib
                .get::<Symbol<'_, CreateFn>>(b"_plugin_create")
                .map_err(|e| format!("Failed to load plugin {}: {}", path.display(), e))?;
            Box::from_raw(ctor())
        };

        debug!("Loaded plugin '{}'", plugin.name());

        self.loaded.push(lib);
        if let Some(logger) = self.logger {
            plugin.set_logger(logger, log::max_level());
        }
        plugin.on_load();
        self.plugins.push(plugin);

        Ok(())
    }

    fn unload_plugins(&mut self) {
        for plugin in self.plugins.drain(..) {
            plugin.on_unload();
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        self.unload_plugins();

        for lib in self.loaded.drain(..) {
            drop(lib);
        }
    }
}
