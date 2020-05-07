mod context;
pub mod event;
pub mod graphic;
pub mod math;
mod scene;
mod window;
pub use bytemuck;
pub use scene::*;
pub use window::*;
mod frame;
pub use frame::*;

async fn start_async<S: 'static + Scene>(scene: S, graphic_options: graphic::GpuOptions) {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            ffi::warn(msg.as_str());
        }));
    }

    let window = crate::Window::new();

    let (gpu, surface) = graphic::create_gpu_surface(window, &graphic_options)
        .await
        .expect("Cannot initialize Gpu.");

    let context = context::Context {
        scene: Box::new(scene),
        gpu,
        surface,
    };

    #[cfg(target_arch = "wasm32")]
    unsafe {
        window::event::Bridge::init(|| context);
    }
}

pub fn start<S: 'static + Scene>(scene: S, graphic_options: graphic::GpuOptions) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(start_async::<S>(scene, graphic_options));
    }
}
