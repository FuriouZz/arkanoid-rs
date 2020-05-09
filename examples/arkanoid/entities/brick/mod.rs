use crate::drawables::Sprite;
use nalgebra::{Matrix4, Vector3};

pub struct Brick {
    sprite: Sprite,
    texture: wgpu::BindGroup,
    m4: Matrix4<f32>,
}

impl Brick {
    pub fn new(gpu: &mut fine::graphic::Gpu) -> Self {
        let sprite = Sprite::new(gpu);
        let view = gpu.create_texture_view(&include_bytes!("../../assets/brick2.png")[..]);
        let texture = sprite.bind_texture(gpu, &view);
        let m4 = Matrix4::identity();

        Self {
            sprite,
            texture,
            m4,
        }
    }
}

impl crate::drawables::Drawable for Brick {
    fn create_pipeline(&mut self, frame: &mut fine::Frame) {}

    fn render_pipeline(&mut self, frame: &mut fine::Frame) {
        self.m4 = self.m4.append_translation(&Vector3::new(0.001, 0.0, 0.0));
        // r.column(0).add_scalar(0.1);
        self.sprite.draw(frame, &self.texture, &self.m4);
    }
}
