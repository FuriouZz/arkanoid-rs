#![deny(unused_results)]
#![allow(unused)]

mod context;
pub mod event;
pub mod graphic;
pub mod math;
mod scene;
mod window;
pub use bytemuck;
pub use scene::Scene;
pub use window::*;
mod frame;
pub use frame::Frame;

async fn start_async<S: 'static + Scene>(graphic_options: graphic::GpuOptions) {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(|info| {
            let msg = format!("{:?}", info);
            ffi::warn(msg.as_str());
        }));
    }

    let window = crate::Window::new();

    let (mut gpu, mut surface) = graphic::gpu::create(window, &graphic_options)
        .await
        .expect("Cannot initialize Gpu.");

    let scene = S::on_load(Frame {
        gpu: &mut gpu,
        surface: &mut surface,
    });

    surface.submit(&mut gpu);

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

pub fn start<S: 'static + Scene>(graphic_options: graphic::GpuOptions) {
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(start_async::<S>(graphic_options));
    }
}
