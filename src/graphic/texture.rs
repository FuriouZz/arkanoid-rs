pub struct Texture2D {
    texture: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn from_image_data(buffer: &[u8], gpu: &mut super::Gpu) -> Self {
        let img = image::load_from_memory(buffer)
            .expect("cannot open image").to_rgba();
        let width = img.width();
        let height = img.height();

        Self::from_bytes(&img.into_raw()[..], width, height, gpu)
    }

    pub fn from_bytes(bytes: &[u8], width: u32, height: u32, gpu: &mut super::Gpu) -> Self {
        // Create wgpu::Buffer
        let copy = gpu
            .device
            .create_buffer_with_data(bytes, wgpu::BufferUsage::COPY_SRC);

        // Create texture
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        // Create texture view
        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: wgpu::TextureFormat::Rgba8Unorm,
            dimension: wgpu::TextureViewDimension::D2,
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            array_layer_count: 1,
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
            texture: view,
            width,
            height,
        }
    }

    pub fn red(gpu: &mut super::Gpu, size: usize) -> Self {
        let v: Vec<u8> = (0..size * size)
            .flat_map(|_index| {
                std::iter::once(0xFF)
                    .chain(std::iter::once(0x00))
                    .chain(std::iter::once(0x00))
                    .chain(std::iter::once(1))
            })
            .collect();

        Self::from_bytes(&v, size as u32, size as u32, gpu)
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.texture
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
