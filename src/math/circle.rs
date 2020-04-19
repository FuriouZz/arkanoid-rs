use super::Rect;
use super::Vec2;

pub struct Circle {
    pub position: Vec2,
    pub radius: f64,
}

impl Circle {
    pub fn new() -> Self {
        Self {
            position: Vec2 { x: 0., y: 0. },
            radius: 0.,
        }
    }

    pub fn get_rect(&self) -> Rect {
        Rect {
            position: Vec2 {
                x: self.position.x - self.radius,
                y: self.position.y - self.radius,
            },
            size: Vec2 {
                x: self.position.x - self.radius + self.radius * 2.,
                y: self.position.y - self.radius + self.radius * 2.,
            },
        }
    }
}
