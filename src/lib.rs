pub mod event;
pub mod ffi;
pub mod wasm;
pub mod math;

use std::panic;

pub fn start<F, T>(handler: F)
where
    T: event::EventHandler + 'static,
    F: FnOnce() -> T,
{
    unsafe {
        wasm::Application::init(handler);
    }

    {
        panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            wasm::console::warn(msg.as_str());
        }));
    }
}
