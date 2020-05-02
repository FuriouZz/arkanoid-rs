use crate::event::{Event, EventHandler};
use crate::Window;

pub struct Context {
    pub window: Window,
    pub scene: Box<dyn crate::Scene>,
    pub gpu: crate::graphic::Gpu,
}

impl Context {}

impl EventHandler for Context {
    fn init(&mut self) {
        self.scene.on_init(&self.window, &self.gpu.device);
    }

    fn on_event(&mut self, e: Event) {
        match e {
            Event::Resize(width, height) => {
                self.window.resize(width, height);
                self.gpu.swap_chain_descriptor.width = width;
                self.gpu.swap_chain_descriptor.height = height;
                self.gpu.swap_chain = self
                    .gpu
                    .device
                    .create_swap_chain(&self.gpu.surface, &self.gpu.swap_chain_descriptor);
            }
            Event::Frame => {
                let frame = self
                    .gpu
                    .swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                let command_buf = self.scene.on_draw(&frame, &self.gpu.device);
                self.gpu.queue.submit(command_buf);
            }
            _ => {}
        }
    }
}
