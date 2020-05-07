use crate::event::Event;

pub(crate) struct Context {
    pub(crate) scene: Box<dyn crate::Scene>,
    pub(crate) gpu: crate::graphic::Gpu,
    pub(crate) surface: crate::graphic::Surface,
}

impl Context {
    pub(crate) fn init(&mut self) {
        self.surface.window.ready();

        let frame = crate::Frame {
            gpu: &mut self.gpu,
            surface: &mut self.surface,
        };
        self.scene.on_init(frame);
        self.surface.submit(&mut self.gpu);
    }

    pub(crate) fn on_event(&mut self, e: Event) {
        match e {
            Event::Resize(width, height) => {
                self.surface.resize(&self.gpu, width, height);
            }
            Event::Frame => {
                let frame = crate::Frame {
                    gpu: &mut self.gpu,
                    surface: &mut self.surface,
                };
                self.scene.on_draw(frame);
                self.surface.submit(&mut self.gpu);
            }
            _ => {}
        }
        self.scene.on_event(e);
    }
}
