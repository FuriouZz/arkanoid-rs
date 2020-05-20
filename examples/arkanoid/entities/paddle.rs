use crate::pipelines::Sprite;
use fine::math::Vector2;
use std::collections::HashSet;

pub struct Paddle {
    pub sprite: Sprite,
    acc: Vector2<f32>,
}

impl Paddle {
    pub fn new(sprite: Sprite) -> Self {
        Self {
            sprite,
            acc: Vector2::new(0.0, 0.0),
        }
    }

    pub fn transform(&self) -> &fine::Transform {
        &self.sprite.transform
    }

    pub fn transform_mut(&mut self) -> &mut fine::Transform {
        &mut self.sprite.transform
    }

    pub fn update(&mut self, keys: &HashSet<fine::event::KeyCode>, dt: f32) {
        if keys.contains(&fine::event::KeyCode::Left) {
            self.acc[0] += 10.0 * dt;
        }
        if keys.contains(&fine::event::KeyCode::Right) {
            self.acc[0] -= 10.0 * dt;
        }

        self.acc[0] *= 0.96;

        let t_paddle = self.sprite.transform.translation_mut();
        t_paddle[0] = t_paddle[0] + self.acc[0];
    }
}