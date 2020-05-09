use crate::event::Event;

pub(crate) struct Context {
    pub(crate) scene: Box<dyn crate::Scene>,
    pub(crate) gpu: crate::graphic::Gpu,
    pub(crate) surface: crate::graphic::Surface,
}

impl Context {
    pub(crate) fn init(&mut self) {
        self.surface.window.ready();

        self.scene.on_start(super::Frame {
            gpu: &mut self.gpu,
            surface: &mut self.surface,
        });

        self.surface.submit(&mut self.gpu);
    }

    pub(crate) fn on_event(&mut self, e: Event) {
        match e {
            Event::Resize(width, height) => {
                self.surface.resize(&self.gpu, width, height);
            }
            Event::Frame => {
                self.scene.on_draw(super::Frame {
                    gpu: &mut self.gpu,
                    surface: &mut self.surface,
                });
                self.surface.submit(&mut self.gpu);
            }
            _ => {}
        }
        self.scene.on_event(e);
    }
}
