#![warn(rust_2018_idioms)]
#![warn(clippy::all)]


use std::{
    env,
    path::PathBuf,
};

use log::info;

use plugin::PluginManager;


macro_rules! loki_logformat {
    ($record:ident, $message:ident) => {
        format_args!(r#"loglevel={} timestamp={} target="{}" content="{}""#,
            $record.level(),
            chrono::Utc::now().format("%+"),
            $record.target(),
            $message
        )
    };
}


fn main() -> Result<(), log::SetLoggerError> {
    fern::Dispatch::new()
        .format(|out, message, record| out.finish(loki_logformat!(record, message)))
        .level(log::LevelFilter::Debug)
        .level_for("hello_world", log::LevelFilter::Debug)
        .level_for("bye_world", log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;

    info!("Logger initialized");

    let mut plugman = PluginManager::new();
    plugman.set_logger(log::logger());
    let plugin_dir = env::var("PTEST_PLUGIN_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| env::current_dir().unwrap().join("plugins"));

    let libs = plugin_dir
        .read_dir()
        .expect("Uuuuh..., something's fucked")
        .map(|x| x.expect("FUCK").path())
        .collect::<Vec<_>>();


    info!("Loading plugins...");
    if let Err(e) = plugman.load_plugins(&libs) {
        eprintln!("{}", e);
    }

    Ok(())
}
