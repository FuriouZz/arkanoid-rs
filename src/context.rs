use crate::event::{EventHandler, Event};
use crate::Window;

pub struct Context {
    pub window: Window,
    pub scene: Box<dyn crate::Scene>,
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub surface: wgpu::Surface,
    pub swapchain: wgpu::SwapChain,
    pub swapchain_descriptor: wgpu::SwapChainDescriptor,
}

impl Context {}

impl EventHandler for Context {
    fn init(&mut self) {
        self.scene.on_init(&self.window, &self.device);
    }

    fn on_event(&mut self, e: Event) {
        match e {
            Event::Resize(width, height) => {
                self.window.size.0 = width;
                self.window.size.1 = height;
                self.swapchain_descriptor.width = width;
                self.swapchain_descriptor.height = height;
                self.swapchain = self.device.create_swap_chain(&self.surface, &self.swapchain_descriptor);
            }
            Event::Frame => {
                let frame = self.swapchain.get_next_texture().expect("Timeout when acquiring next swap chain texture");
                let command_buf = self.scene.on_draw(&frame, &self.device);
                self.queue.submit(command_buf);
            }
            _ => {}
        }
    }
}