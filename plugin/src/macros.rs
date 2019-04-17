#[macro_export]
macro_rules! define_plugin {
    ($type:ty, $ctor:path) => {
        #[no_mangle]
        pub extern "C" fn _plugin_create() -> *mut dyn $crate::Plugin {
            let ctor: fn() -> $type = $ctor;
            let boxed: Box<$crate::Plugin> = Box::new(ctor());
            Box::into_raw(boxed)
        }
    };
}
