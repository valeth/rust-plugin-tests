use plugin::{Plugin, plugin};


#[derive(Default)]
pub struct HelloWorldPlugin;

impl Plugin for HelloWorldPlugin {
    fn name(&self) -> &'static str {
        "hello_world"
    }

    fn on_load(&self) {
        // println!("Hello World");
    }
}


plugin!(HelloWorldPlugin, Default::default);
