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
    pub surface: wgpu::Surface,
    pub swap_chain: wgpu::SwapChain,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub queue: wgpu::Queue,
}

impl Gpu {
    pub async fn create(window: &crate::window::Window, options: &GpuOptions) -> Option<Self> {
        let instance = wgpu::Instance::new();

        // Create a surface to draw
        let surface = unsafe { instance.create_surface(window) };

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

        Some(Self {
            device,
            surface,
            swap_chain_descriptor,
            swap_chain,
            queue,
        })
    }

    pub fn create_shader_module(self: &Self, source: &[u8]) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&source)).unwrap())
    }
}
