use crate::pipelines::{SpritePipeline, SpriteInstance};
use fine::graphic::Texture2D;
use nalgebra::Matrix4;

pub struct Brick {
    sprite: SpriteInstance,
    transform: Matrix4<f32>,
}

impl Brick {
    pub fn new(gpu: &mut fine::graphic::Gpu, pipeline: &SpritePipeline) -> Self {
        let view = Texture2D::from_image_data(&include_bytes!("../assets/brick2.png")[..], gpu);
        // let view = Texture2D::red(gpu, 64);
        let transform = Matrix4::identity().append_scaling(1.0);
        let sprite = pipeline.create_sprite_instance(gpu, view);

        Self { sprite, transform }
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.transform[12] = x * 2.0 * self.sprite.texture.width() as f32;
        self.transform[13] = y * 2.0 * self.sprite.texture.height() as f32;
    }
    pub fn update(&mut self, frame: &mut fine::Frame, camera: &crate::camera::Camera) -> &SpriteInstance {
        let gpu = frame.gpu();
        self.sprite.update_transform(gpu, &camera.model_view_projection(&self.transform));
        &self.sprite
    }
}
