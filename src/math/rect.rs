use super::Vec2;
use super::Line;

#[derive(Debug)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            position: Vec2 { x: 0., y: 0. },
            size: Vec2 { x: 0., y: 0. },
        }
    }

    pub fn left(&self) -> f64 {
        self.position.x
    }

    pub fn right(&self) -> f64 {
        self.position.x + self.size.x
    }

    pub fn top(&self) -> f64 {
        self.position.y
    }

    pub fn bottom(&self) -> f64 {
        self.position.y + self.size.y
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2 { x: self.position.x, y: self.position.y }
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2 { x: self.position.x + self.size.x, y: self.position.y }
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2 { x: self.position.x, y: self.position.y + self.size.y }
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2 { x: self.position.x + self.size.x, y: self.position.y + self.size.y }
    }

    pub fn edge_top(&self) -> Line {
        Line { v0: self.top_left(), v1: self.top_right() }
    }

    pub fn edge_bottom(&self) -> Line {
        Line { v0: self.bottom_left(), v1: self.bottom_right() }
    }

    pub fn edge_left(&self) -> Line {
        Line { v0: self.top_left(), v1: self.bottom_left() }
    }

    pub fn edge_right(&self) -> Line {
        Line { v0: self.top_right(), v1: self.bottom_right() }
    }

}
