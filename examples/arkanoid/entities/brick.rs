use crate::pipelines::SpriteClip;
use fine::graphic::TextureAtlas;
use std::rc::Rc;

pub struct Brick {
    pub sprite: SpriteClip,
}

impl Brick {
    pub fn new(atlas: Rc<TextureAtlas>) -> Self {
        let mut sprite = SpriteClip::new("blue", atlas);
        // sprite.set_origin(0.0, 0.0);
        sprite.scale(0.5);
        Self { sprite }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
