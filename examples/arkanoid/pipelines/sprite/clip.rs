use super::Sprite;
use std::rc::Rc;
use fine::graphic::TextureAtlas;

pub struct SpriteClip {
    sprite: Sprite,
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

    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.sprite.set_origin(x, y);
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.sprite.set_position(x, y)
    }

    pub fn scale(&mut self, s: f32) {
        self.sprite.scale(s);
    }

    pub fn rotate(&mut self, rad: f32) {
        self.sprite.rotate(rad);
    }

    pub fn x(&self) -> f32 {
        self.sprite.x()
    }

    pub fn y(&self) -> f32 {
        self.sprite.y()
    }

    pub fn width(&self) -> f32 {
        self.sprite.width()
    }

    pub fn height(&self) -> f32 {
        self.sprite.height()
    }

    pub fn sprite(&self) -> &Sprite {
        &self.sprite
    }
}