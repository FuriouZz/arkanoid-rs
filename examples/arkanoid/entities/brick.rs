use crate::pipelines::Sprite;
use fine::graphic::Texture2DAtlas;

pub struct Brick {
    pub sprite: Sprite,
}

impl Brick {
    pub fn new(atlas: &Texture2DAtlas) -> Self {
        let sprite = Sprite::from_atlas(atlas, 0);
        Self { sprite }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
