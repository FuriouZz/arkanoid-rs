pub struct Texture2D {
    texture: wgpu::TextureView,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn from_image(gpu: &mut super::Gpu, buffer: &[u8]) -> Self {
        let img = image::load_from_memory(buffer)
            .expect("cannot open image")
            .to_bgra();
        let width = img.width();
        let height = img.height();

        Self::from_bytes(gpu, width, height, &img.into_raw()[..])
    }

    pub fn from_bytes(gpu: &mut super::Gpu, width: u32, height: u32, bytes: &[u8]) -> Self {
        let texture = create_texture(gpu, width, height, bytes);

        Self {
            texture,
            width,
            height,
        }
    }

    pub fn from_images(gpu: &mut super::Gpu, buffers: &[&[u8]]) -> Self {
        let mut width = 0;
        let mut height = 0;

        let mut bytes: Vec<u8> = Vec::new();

        for buffer in buffers {
            let img = image::load_from_memory(buffer)
                .expect("cannot open image")
                .to_bgra();
            width = img.width();
            height = img.height();
            bytes.extend_from_slice(&img.into_raw()[..]);
        }

        Self::from_bytes_array(gpu, width, height, buffers.len() as u32, &bytes[..])
    }

    pub fn from_packed_images(
        gpu: &mut super::Gpu,
        width: u32,
        height: u32,
        layout_count: u32,
        buffer: &[u8],
    ) -> Self {
        let img = image::load_from_memory(buffer)
            .expect("cannot open image")
            .to_bgra();

        Self::from_bytes_array(gpu, width, height, layout_count, &img.into_raw()[..])
    }

    pub fn from_bytes_array(
        gpu: &mut super::Gpu,
        width: u32,
        height: u32,
        layout_count: u32,
        bytes: &[u8],
    ) -> Self {
        let texture = create_texture_array(gpu, width, height, layout_count, bytes);

        Self {
            texture,
            width,
            height,
        }
    }

    pub fn red(gpu: &mut super::Gpu, size: usize) -> Self {
        let bytes: Vec<u8> = (0..size * size)
            .flat_map(|_index| {
                std::iter::once(0xFF)
                    .chain(std::iter::once(0x00))
                    .chain(std::iter::once(0x00))
                    .chain(std::iter::once(1))
            })
            .collect();

        Self::from_bytes(gpu, size as u32, size as u32, &bytes)
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

fn create_texture(
    gpu: &mut super::Gpu,
    width: u32,
    height: u32,
    bytes: &[u8],
) -> wgpu::TextureView {
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
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
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

    // Create texture view
    texture.create_view(&wgpu::TextureViewDescriptor {
        format: wgpu::TextureFormat::Bgra8Unorm,
        dimension: wgpu::TextureViewDimension::D2,
        aspect: wgpu::TextureAspect::default(),
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        array_layer_count: 1,
    })
}

fn create_texture_array(
    gpu: &mut super::Gpu,
    width: u32,
    height: u32,
    layer_count: u32,
    bytes: &[u8],
) -> wgpu::TextureView {
    // Create texture
    let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        format: wgpu::TextureFormat::Bgra8Unorm,
        dimension: wgpu::TextureDimension::D2,
        size: wgpu::Extent3d {
            width,
            height,
            depth: layer_count,
        },
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    // Create wgpu::Buffer
    let copy = gpu
        .device
        .create_buffer_with_data(bytes, wgpu::BufferUsage::COPY_SRC);

    for i in 0..layer_count {
        gpu.encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &copy,
                offset: (i * 4 * width * height) as u64,
                bytes_per_row: 4 * width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: i,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
        );
    }

    // Create texture view
    texture.create_view(&wgpu::TextureViewDescriptor {
        format: wgpu::TextureFormat::Bgra8Unorm,
        dimension: wgpu::TextureViewDimension::D2Array,
        aspect: wgpu::TextureAspect::default(),
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        array_layer_count: layer_count,
    })
}
