pub struct Shader {
    module: wgpu::ShaderModule,
}

impl Shader {
    pub fn from_bytes(gpu: &super::Gpu, bytes: &[u8]) -> std::io::Result<Shader> {
        wgpu::read_spirv(std::io::Cursor::new(&bytes)).map(|spv| {
            let module = gpu.device.create_shader_module(&spv);
            Self { module }
        })
    }

    pub fn stage<'a>(&'a self, entry_point: &'a str) -> wgpu::ProgrammableStageDescriptor<'a> {
        wgpu::ProgrammableStageDescriptor {
            module: &self.module,
            entry_point,
        }
    }
}
