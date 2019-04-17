use plugin::{Plugin, plugin};


#[derive(Default)]
pub struct ByeWorldPlugin;

impl Plugin for ByeWorldPlugin {
    fn name(&self) -> &'static str {
        "bye_world"
    }

    fn on_load(&self) {
        // println somehow leaks memory
        // println!("Bye World");
    }
}


plugin!(ByeWorldPlugin, Default::default);
