#[rustfmt::skip]

pub mod blend;
pub mod utils;
pub mod vertex_attribute;
mod binding;
pub use binding::BindingDescriptor;
pub use binding::BindingLayout;
mod surface;
pub use surface::Surface;
pub(crate) mod gpu;
pub use gpu::Gpu;
pub use gpu::GpuOptions;
pub(crate) use gpu::create;
pub use wgpu;
mod texture;
pub use texture::Texture2D;

pub const DEFAULT_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
