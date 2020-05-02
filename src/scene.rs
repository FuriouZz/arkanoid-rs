pub trait Scene {
    fn on_init(&mut self, window: &crate::Window, device: &wgpu::Device);
    fn on_event(&mut self, e: crate::event::Event);
    fn on_draw(
        &mut self,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> Option<wgpu::CommandBuffer>;
}
