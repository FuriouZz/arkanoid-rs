use super::Vec2;

pub struct Line {
    pub v0: Vec2,
    pub v1: Vec2,
}

impl Line {
    pub fn new() -> Self {
        Self {
            v0: Vec2 { x: 0., y: 0. },
            v1: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn length(&self) -> f64 {
        self.v0.distance(&self.v1)
    }
}
