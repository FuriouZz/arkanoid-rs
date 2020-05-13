use crate::pipelines::Sprite;
use fine::graphic::TextureAtlas;

pub struct Brick {
    pub sprite: Sprite,
}

impl Brick {
    pub fn new(atlas: &TextureAtlas) -> Self {
        let sprite = Sprite::from_atlas(atlas, "blue");
        Self { sprite }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
