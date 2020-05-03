mod binding;
pub mod blend;
mod utils;
pub mod vertex_attribute;
pub use binding::*;
pub use wgpu;
mod gpu;
pub use gpu::*;

pub const DEFAULT_TEXTURE_FORMAT: wgpu::TextureFormat =
    wgpu::TextureFormat::Bgra8Unorm;