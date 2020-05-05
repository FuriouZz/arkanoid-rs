use crate::drawables::Sprite;

pub struct Brick {
    sprite: Option<Sprite>,
}

impl Brick {
    pub fn new() -> Self {
        Self { sprite: None }
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

        let quad = Sprite::new(device, &mut encoder);
        self.sprite = Some(quad);

        let quad = self.sprite.as_ref().unwrap();
        quad.draw(&mut encoder, &frame.view);
        Some(encoder.finish())
    }

    fn render_pipeline(
        &self,
        device: &fine::graphic::wgpu::Device,
        frame: &fine::graphic::wgpu::SwapChainOutput,
    ) -> Option<fine::graphic::wgpu::CommandBuffer> {
        self.sprite.as_ref().map(|drawable| {
            let mut encoder =
                device.create_command_encoder(&fine::graphic::wgpu::CommandEncoderDescriptor {
                    label: None,
                });

            drawable.draw(&mut encoder, &frame.view);
            encoder.finish()
        })
    }
}
