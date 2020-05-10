use super::Brick;
use fine::graphic::Gpu;
use crate::pipelines::Sprite;

pub struct Level {
    pub bricks: Vec<Brick>,
}

impl Level {
    pub fn generate(width: u32, height: u32, gpu: &mut Gpu, sprite: &Sprite) -> Self {
        let bricks: Vec<Brick> = (0..width*height).map(|index| {
            let x = (index % width) as f32;
            let y = f32::floor((index as f32) / (height as f32));
            let mut brick = Brick::new(gpu, sprite);
            brick.set_position(x, y);
            brick
        }).collect();
        Self {
            bricks
        }
    }
}