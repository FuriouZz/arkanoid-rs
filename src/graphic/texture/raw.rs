use crate::graphic::Gpu;
use image::DynamicImage;

pub struct RawTexture2D {
    texture: wgpu::Texture,
    width: u32,
    height: u32,
}

impl RawTexture2D {
    pub fn from_bytes(gpu: &mut Gpu, width: u32, height: u32, usage: wgpu::TextureUsage, bytes: &[u8]) -> Self {
        // Create wgpu::Buffer
        let copy = gpu
            .device
            .create_buffer_with_data(bytes, wgpu::BufferUsage::COPY_SRC);

        // Create texture
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureDimension::D2,
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsage::COPY_DST | usage,
        });

        gpu.encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &copy,
                offset: 0,
                bytes_per_row: 4 * width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
        );

        Self {
            texture,
            width,
            height,
        }
    }

    pub fn from_image(gpu: &mut Gpu, usage: wgpu::TextureUsage, img: &DynamicImage) -> Self {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        Self::from_bytes(gpu, width, height, usage, bytes)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn as_raw(&self) -> &wgpu::Texture {
        &self.texture
    }
}