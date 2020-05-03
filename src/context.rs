use crate::event::Event;
use crate::Window;
use wasm_bindgen::describe::WasmDescribe;

pub(crate) struct Context {
    pub(crate) window: Window,
    pub(crate) scene: Box<dyn crate::Scene>,
    pub(crate) gpu: crate::graphic::Gpu,
}

impl Context {
    pub(crate) fn init(&mut self) {
        self.window.ready();
        self.scene.on_init(&self.window, &self.gpu.device);
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
                let command_buf = self.scene.on_draw(&frame, &self.gpu.device);
                self.gpu.queue.submit(command_buf);
            }
            _ => {}
        }
    }
}
