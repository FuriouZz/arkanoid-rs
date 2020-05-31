use crate::pipelines::Sprite;

pub struct Brick {
    pub sprite: Sprite,
    pub alive: bool,
}

impl Brick {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            alive: true,
        }
    }

    pub fn transform(&mut self) -> &mut fine::Transform {
        &mut self.sprite.transform
    }
}