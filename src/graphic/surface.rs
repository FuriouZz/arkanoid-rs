use super::Gpu;

pub struct Surface {
    pub(crate) window: crate::Window,
    pub(crate) surface: wgpu::Surface,
    pub(crate) swap_chain: wgpu::SwapChain,
    pub(crate) swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub(crate) output: Option<wgpu::SwapChainOutput>,
}

impl Surface {
    pub fn frame(&mut self) -> &wgpu::TextureView {
        if self.output.is_none() {
            let frame = self
                .swap_chain
                .get_next_texture()
                .expect("Timeout when acquiring next swap chain texture");
            self.output = Some(frame);
        }
        &self.output.as_ref().unwrap().view
    }

    pub fn resize(&mut self, gpu: &Gpu, width: u32, height: u32) {
        self.window.resize(width, height);
        self.swap_chain_descriptor.width = width;
        self.swap_chain_descriptor.height = height;
        self.swap_chain = gpu
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
        self.output = None;
    }

    pub fn submit(&mut self, gpu: &mut Gpu) {
        let encoder = gpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        let current = std::mem::replace(&mut gpu.encoder, encoder);
        gpu.queue.submit(Some(current.finish()));
        self.output = None;
    }
}
