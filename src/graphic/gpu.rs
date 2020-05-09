pub struct GpuOptions {
    /// Power preference
    pub power_preference: wgpu::PowerPreference,
    /// Device options
    pub device: wgpu::DeviceDescriptor,
    /// Preferred backend
    pub backends: wgpu::BackendBit,
    /// Swap chain texture usage
    pub usage: wgpu::TextureUsage,
    /// Swap chain texture format
    pub format: wgpu::TextureFormat,
    /// Swap chain present mode
    pub present_mode: wgpu::PresentMode,
}

impl Default for GpuOptions {
    fn default() -> Self {
        Self {
            power_preference: wgpu::PowerPreference::Default,
            backends: wgpu::BackendBit::PRIMARY,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: super::DEFAULT_TEXTURE_FORMAT,
            present_mode: wgpu::PresentMode::Mailbox,
            device: wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            },
        }
    }
}

pub struct Gpu {
    pub device: wgpu::Device,
    pub encoder: wgpu::CommandEncoder,
    pub(crate) queue: wgpu::Queue,
}

impl Gpu {
    // Create a shader module from SPIRV source
    pub fn create_shader_module(&self, source: &[u8]) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&source)).unwrap())
    }

    // Create a buffer
    pub fn create_buffer<A: bytemuck::Pod>(
        &self,
        vertices: &[A],
        usage: wgpu::BufferUsage,
    ) -> wgpu::Buffer {
        self.device
            .create_buffer_with_data(bytemuck::cast_slice(vertices), usage)
    }

    // Create a D2 texture view
    pub fn create_texture_view(&mut self, source: &[u8]) -> wgpu::TextureView {
        // Load bytes
        let img = image::load_from_memory(source)
            .expect("cannot open image")
            .to_rgba();
        let width = img.width();
        let height = img.height();

        // Create wgpu::Buffer
        let copy = self
            .device
            .create_buffer_with_data(&img.into_raw()[..], wgpu::BufferUsage::COPY_SRC);

        // Create texture
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
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

        // Send a command to copy image buffer to the texture
        self.encoder.copy_buffer_to_texture(
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

        view
    }
}

pub(crate) async fn create(
    window: crate::Window,
    options: &GpuOptions,
) -> Option<(Gpu, super::Surface)> {
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

    let surf = super::Surface {
        window,
        surface,
        swap_chain_descriptor,
        swap_chain,
        output: None,
    };

    Some((gpu, surf))
}