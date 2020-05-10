use crate::pipelines::{Sprite, SpritePipeline};
use fine::graphic::Texture2D;

pub struct Brick {
    pub sprite: Sprite,
}

impl Brick {
    pub fn new(gpu: &mut fine::graphic::Gpu, pipeline: &SpritePipeline) -> Self {
        let view = Texture2D::from_image(gpu, &include_bytes!("../assets/brick2.png")[..]);
        let sprite = pipeline.create_sprite(gpu, view);

        Self { sprite }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
