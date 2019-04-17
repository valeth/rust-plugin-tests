#![warn(rust_2018_idioms)]

mod plugin_manager;


use std::{
    env,
    path::PathBuf,
};

use self::plugin_manager::PluginManager;


fn main() {
    let mut plugman = PluginManager::new();
    let plugin_dir = env::var("PTEST_PLUGIN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap().join("plugins"));

    let libs = plugin_dir
        .read_dir()
        .expect("Uuuuh..., something's fucked")
        .map(|x| x.expect("FUCK").path())
        .collect::<Vec<_>>();


    println!("Loading plugins...");
    if let Err(e) = plugman.load_plugins(&libs) {
        eprintln!("{}", e);
    }
}
