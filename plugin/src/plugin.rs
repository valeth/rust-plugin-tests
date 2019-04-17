use std::any::Any;


pub type CreateFn = unsafe extern fn() -> *mut dyn Plugin;


pub trait Plugin: Any + Send + Sync {
    /// A short name for this plugin
    fn name(&self) -> &'static str;

    /// Handler being run right after plugin load
    fn on_load(&self) {}

    /// Handler being run just before dropping the plugin
    fn on_unload(&self) {}

    /// Set the logger for this plugin's environment
    fn set_logger(&self, logger: &'static dyn log::Log, level: log::LevelFilter) {
        if let Err(e) = log::set_logger(logger) {
            eprintln!("Failed to set logger: {}", e);
        }
        log::set_max_level(level);
    }
}
