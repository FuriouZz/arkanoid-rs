use crate::pipelines::SpriteClip;
use fine::graphic::TextureAtlas;
use std::rc::Rc;

pub struct Brick {
    pub clip: SpriteClip,
}

impl Brick {
    pub fn new(atlas: Rc<TextureAtlas>) -> Self {
        let mut clip = SpriteClip::new("blue", atlas);
        clip.sprite.transform.scale(0.5);
        Self { clip }
    }
    pub fn update(&mut self, frame: &mut fine::Frame) {}
}
