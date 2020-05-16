pub struct Frame<'a> {
    pub(crate) gpu: &'a mut crate::graphic::Gpu,
    pub(crate) surface: &'a mut crate::graphic::Surface,
}

impl<'a> Frame<'a> {
    pub fn width(&self) -> u32 {
        self.surface.window.width()
    }

    pub fn height(&self) -> u32 {
        self.surface.window.height()
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.surface.window.width(), self.surface.window.height())
    }

    pub fn gpu(&mut self) -> &mut crate::graphic::Gpu {
        self.gpu
    }

    pub fn target(&mut self) -> (&mut crate::graphic::Gpu, &wgpu::TextureView) {
        (&mut self.gpu, self.surface.frame())
    }
}
