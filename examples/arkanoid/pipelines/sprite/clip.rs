use super::Sprite;
use fine::graphic::TextureAtlas;
use std::rc::Rc;

pub struct SpriteClip {
    pub sprite: Sprite,
    atlas: Rc<TextureAtlas>,
}

impl SpriteClip {
    pub fn new(name: &str, atlas: Rc<TextureAtlas>) -> Self {
        let sprite = Sprite::from_atlas(name, atlas.as_ref());
        Self { sprite, atlas }
    }

    pub fn set_frame(&mut self, name: &str) {
        self.sprite.set_frame_from_atlas(name, self.atlas.as_ref());
    }
}
