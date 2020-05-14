use super::Sprite;
use std::rc::Rc;
use fine::graphic::TextureAtlas;

pub struct SpriteClip {
    pub sprite: Sprite,
    atlas: Rc<TextureAtlas>,
}

impl SpriteClip {
    pub fn new(name: &str, atlas: Rc<TextureAtlas>) -> Self {
        let sprite = Sprite::from_atlas(name, atlas.as_ref());
        Self {
            sprite,
            atlas,
        }
    }

    pub fn set_frame(&mut self, name: &str) {
        self.sprite.set_frame_from_atlas(name, self.atlas.as_ref());
    }
}