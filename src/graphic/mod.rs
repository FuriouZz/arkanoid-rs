#[rustfmt::skip]

pub mod blend;
pub mod utils;
pub mod vertex_attribute;
mod binding;
pub use binding::*;
mod surface;
pub use surface::*;
pub(crate) mod gpu;
pub use gpu::*;
pub use wgpu;

pub const DEFAULT_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
