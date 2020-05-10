use crate::pipelines::{Sprite, SpritePipeline};
use fine::graphic::Texture2D;

pub struct Brick {
    pub sprite: Sprite,
}

impl Brick {
    pub fn new(gpu: &mut fine::graphic::Gpu, pipeline: &SpritePipeline, texture: &Texture2D) -> Self {
        let sprite = pipeline.create_sprite(gpu, texture);

        Self { sprite }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
