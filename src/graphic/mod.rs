mod binding;
pub mod blend;
mod utils;
pub mod vertex_attribute;
pub use binding::*;
mod surface;
pub use surface::*;
mod gpu;
pub use gpu::*;
pub use wgpu;

pub const DEFAULT_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

pub(crate) async fn create_gpu_surface(
    window: crate::Window,
    options: &GpuOptions,
) -> Option<(Gpu, Surface)> {
    let instance = wgpu::Instance::new();

    // Create a surface to draw
    let surface = unsafe { instance.create_surface(&window) };

    // Request the more appropriate adapter
    let adapter = instance
        .request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: options.power_preference,
                compatible_surface: Some(&surface),
            },
            options.backends,
        )
        .await?;

    // Request device and the render queue
    let (device, queue) = adapter.request_device(&options.device).await.ok()?;

    let swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: options.usage,
        format: options.format,
        width: window.width(),
        height: window.height(),
        present_mode: options.present_mode,
    };
    let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let gpu = Gpu {
        device,
        queue,
        encoder,
    };

    let surf = Surface {
        window,
        surface,
        swap_chain_descriptor,
        swap_chain,
        output: None,
    };

    Some((gpu, surf))
}
