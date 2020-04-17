pub mod event;
pub mod wasm;

use std::panic;

pub fn start<T>(handler: T)
where
    T: event::EventHandler + 'static,
 {
    unsafe {
        wasm::application::Application::init(Box::new(handler));
    }

    {
        panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            wasm::console::warn(msg.as_str());
        }));
    }
}