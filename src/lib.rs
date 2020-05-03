mod context;
pub mod event;
pub mod graphic;
pub mod math;
mod scene;
mod window;
pub use scene::*;
pub use window::*;

async fn start_async<S: 'static + Scene>(scene: S, graphic_options: graphic::GpuOptions) {
    {
        std::panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            ffi::warn(msg.as_str());
        }));
    }

    let window = Window::new();

    let gpu = graphic::Gpu::create(&window, &graphic_options)
    .await
    .expect("Cannot initialize Gpu.");

    let context = context::Context {
        window,
        scene: Box::new(scene),
        gpu,
    };

    unsafe {
        window::event::Bridge::init(|| context);
    }
}

pub fn start<S: 'static + Scene>(scene: S, graphic_options: graphic::GpuOptions) {
    wasm_bindgen_futures::spawn_local(start_async::<S>(scene, graphic_options));
}
