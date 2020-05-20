use crate::pipelines::Sprite;

pub struct Brick {
    pub sprite: Sprite,
}

impl Brick {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite
        }
    }

    pub fn transform(&mut self) -> &mut fine::Transform {
        &mut self.sprite.transform
    }
}