use std::any::Any;


#[macro_export]
macro_rules! plugin {
    ($type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern fn _plugin_create() -> *mut dyn $crate::Plugin {
            let ctor: fn() -> $type = $constructor;
            let object = ctor();
            let boxed: Box<$crate::Plugin> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}


pub type CreateFn = extern fn() -> *mut dyn Plugin;


pub trait Plugin: Any + Send + Sync {
    fn name(&self) -> &'static str;

    fn on_load(&self) {}

    fn on_unload(&self) {}
}
