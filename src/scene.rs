pub trait Scene {
    fn on_init(
        &mut self,
        window: &crate::Window,
        device: &wgpu::Device,
        frame: &wgpu::SwapChainOutput,
    ) -> Option<wgpu::CommandBuffer>;
    fn on_event(&mut self, e: crate::event::Event);
    fn on_draw(
        &mut self,
        device: &wgpu::Device,
        frame: &wgpu::SwapChainOutput,
    ) -> Option<wgpu::CommandBuffer>;
}
