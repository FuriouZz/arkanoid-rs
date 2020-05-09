use crate::pipelines::{Sprite, SpriteTexture};
use nalgebra::Matrix4;
use fine::graphic::Texture2D;

pub struct Brick {
    texture: SpriteTexture,
    transform: Matrix4<f32>,
}

impl Brick {
    pub fn new(gpu: &mut fine::graphic::Gpu, sprite: &Sprite) -> Self {
        // let view = Texture2D::from_image_data(
        //     &include_bytes!("../assets/brick2.png")[..],
        //     gpu,
        // );
        let view = Texture2D::red(gpu, 64);
        let texture = sprite.create_sprite_texture(gpu, view);
        let mut transform = Matrix4::identity();
        transform.append_scaling_mut(1.0);
        fine::log!("{}x{}", texture.1.width(), texture.1.height());

        Self { texture, transform }
    }
    pub fn update(&mut self, camera: &crate::camera::Camera) -> (Matrix4<f32>, &SpriteTexture) {
        self.transform[12] = f32::cos((fine::now() as f32) * 0.001);
        (camera.model_view_projection(&self.transform), &self.texture)
    }
}
