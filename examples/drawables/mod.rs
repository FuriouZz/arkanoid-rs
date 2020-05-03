mod triangle;
pub use triangle::*;
mod quad;
pub use quad::*;

pub trait Drawable {
    fn create_pipeline(
        &mut self,
        device: &fine::graphic::wgpu::Device,
        frame: &fine::graphic::wgpu::SwapChainOutput,
    ) -> Option<fine::graphic::wgpu::CommandBuffer>;
    fn render_pipeline(
        &self,
        device: &fine::graphic::wgpu::Device,
        frame: &fine::graphic::wgpu::SwapChainOutput,
    ) -> Option<fine::graphic::wgpu::CommandBuffer>;
}
