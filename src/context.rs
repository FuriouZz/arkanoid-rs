use crate::event::Event;
use crate::Window;

pub(crate) struct Context {
    pub(crate) window: Window,
    pub(crate) scene: Box<dyn crate::Scene>,
    pub(crate) gpu: crate::graphic::Gpu,
}

impl Context {
    pub(crate) fn init(&mut self) {
        self.window.ready();

        let frame = self
            .gpu
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");

        let command_buf = self.scene.on_init(&self.window, &self.gpu.device, &frame);
        self.gpu.queue.submit(command_buf);
    }

    pub(crate) fn on_event(&mut self, e: Event) {
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
                let command_buf = self.scene.on_draw(&self.gpu.device, &frame);
                self.gpu.queue.submit(command_buf);
            }
            _ => {}
        }
        self.scene.on_event(e);
    }
}
