mod binding;
pub mod blend;
mod utils;
mod vertex_buffer;
pub use binding::*;
pub use vertex_buffer::defaults as VertexAttributeDescriptors;
pub use vertex_buffer::VertexAttributeDescriptor;
pub use wgpu;

// Initialize wgpu
pub async fn init_wgpu(
    window: &crate::window::Window,
) -> (
    wgpu::Device,
    wgpu::Surface,
    wgpu::SwapChainDescriptor,
    wgpu::SwapChain,
    wgpu::Queue,
) {
    let instance = wgpu::Instance::new();

    // Create a surface to draw
    let surface = unsafe { instance.create_surface(window) };

    // Request the more appropriate adapter
    let adapter = instance
        .request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .expect("Adapter::request failed");

    // Request device and the render queue
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await
        .unwrap();

    let swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: window.size.0,
        height: window.size.1,
        present_mode: wgpu::PresentMode::Mailbox,
    };
    let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

    (device, surface, swap_chain_descriptor, swap_chain, queue)
}

pub fn create_shader_module(device: &wgpu::Device, source: &[u8]) -> wgpu::ShaderModule {
    device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&source)).unwrap())
}
