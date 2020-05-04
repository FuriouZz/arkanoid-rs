use crate::drawables::Quad;

pub struct Brick {
    quad: Option<Quad>,
}

impl Brick {
    pub fn new() -> Self {
        Self { quad: None }
    }
}

impl crate::drawables::Drawable for Brick {
    fn create_pipeline(
        &mut self,
        device: &fine::graphic::wgpu::Device,
        frame: &fine::graphic::wgpu::SwapChainOutput,
    ) -> Option<fine::graphic::wgpu::CommandBuffer> {
        let mut encoder = device
            .create_command_encoder(&fine::graphic::wgpu::CommandEncoderDescriptor { label: None });

        let quad = Quad::new(device, &mut encoder);
        self.quad = Some(quad);

        let quad = self.quad.as_ref().unwrap();
        quad.draw(&mut encoder, &frame.view);
        Some(encoder.finish())
    }

    fn render_pipeline(
        &self,
        device: &fine::graphic::wgpu::Device,
        frame: &fine::graphic::wgpu::SwapChainOutput,
    ) -> Option<fine::graphic::wgpu::CommandBuffer> {
        self.quad.as_ref().map(|drawable| {
            let mut encoder =
                device.create_command_encoder(&fine::graphic::wgpu::CommandEncoderDescriptor {
                    label: None,
                });

            drawable.draw(&mut encoder, &frame.view);
            encoder.finish()
        })
    }
}
