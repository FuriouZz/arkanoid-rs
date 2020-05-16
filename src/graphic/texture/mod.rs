mod atlas;
pub use atlas::TextureAtlas;
mod texture;
pub use texture::Texture;
mod raw;
pub use raw::RawTexture;

pub trait AsTextureView {
    fn view_rect(&self, frame: Option<String>) -> (u32, nalgebra::Vector4<f32>);
    fn as_view(&self) -> &wgpu::TextureView;
}

pub fn create_texture_color(
    color: u32,
    width: u32,
    height: u32,
    gpu: &mut crate::graphic::Gpu,
) -> RawTexture {
    let bytes: Vec<u8> = (0..width * height)
        .flat_map(|_index| {
            std::iter::once((color & 0xFF) as u8)
                .chain(std::iter::once((color >> 8 & 0xFF) as u8))
                .chain(std::iter::once((color >> 16 & 0xFF) as u8))
                .chain(std::iter::once(0xFF))
        })
        .collect();

    RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsage::COPY_SRC, &bytes)
}
