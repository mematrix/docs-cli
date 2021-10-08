use std::sync::Once;

use tokio::runtime::Runtime;


static RT_INIT_GUARD: Once = Once::new();
static mut RT_INSTANCE: Option<Runtime> = None;


pub fn get_tokio_rt() -> &'static Runtime {
    // SAFETY: `mut RT_INSTANCE` is accessing in a synchronized fashion (Once.call_once),
    // and will otherwise read a immutable ref from `&RT_INSTANCE`.
    unsafe {
        RT_INIT_GUARD.call_once(|| {
            RT_INSTANCE = Runtime::new().ok();
        });

        RT_INSTANCE.as_ref().unwrap()
    }
}

pub fn drop_tokio_rt() {
    // SAFETY: function will be called only on the end of `main`. And guard that it is
    // not be called simultaneously with `get_tokio_rt`.
    unsafe {
        RT_INSTANCE = None;
    }
}
