mod context;
pub mod event;
pub mod graphic;
pub mod math;
pub mod utils;
mod window;
mod scene;
pub use scene::*;
pub use window::*;

// pub fn start<F, T>(handler: F)
// where
//     T: event::EventHandler + 'static,
//     F: FnOnce() -> T,
// {

async fn start_async<S: 'static + Scene>(scene: S) {
    let window = Window::new();

    let (device, surface, swapchain_descriptor, swapchain, queue) =
        graphic::init_wgpu(&window).await;

    let context = context::Context {
        window,
        scene: Box::new(scene),
        queue,
        device,
        surface,
        swapchain,
        swapchain_descriptor,
    };

    unsafe {
        window::event::Bridge::init(|| context);
    }

    {
        std::panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            window::ffi::warn(msg.as_str());
        }));
    }
}

pub fn start<S: 'static + Scene>(scene: S) {
    wasm_bindgen_futures::spawn_local(start_async::<S>(scene));
}
