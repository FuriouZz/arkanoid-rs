use super::Line;
use super::Size;
use super::Vec2;

#[derive(Debug)]
pub struct Rect {
    pub position: Vec2,
    pub size: Size,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            position: Vec2 { x: 0., y: 0. },
            size: Size {
                width: 0.,
                height: 0.,
            },
        }
    }

    pub fn left(&self) -> f32 {
        self.position.x
    }

    pub fn right(&self) -> f32 {
        self.position.x + self.size.width
    }

    pub fn top(&self) -> f32 {
        self.position.y
    }

    pub fn bottom(&self) -> f32 {
        self.position.y + self.size.height
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2 {
            x: self.position.x,
            y: self.position.y,
        }
    }

    pub fn top_right(&self) -> Vec2 {
        Vec2 {
            x: self.position.x + self.size.width,
            y: self.position.y,
        }
    }

    pub fn bottom_left(&self) -> Vec2 {
        Vec2 {
            x: self.position.x,
            y: self.position.y + self.size.height,
        }
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2 {
            x: self.position.x + self.size.width,
            y: self.position.y + self.size.height,
        }
    }

    pub fn edge_top(&self) -> Line {
        Line {
            v0: self.top_left(),
            v1: self.top_right(),
        }
    }

    pub fn edge_bottom(&self) -> Line {
        Line {
            v0: self.bottom_left(),
            v1: self.bottom_right(),
        }
    }

    pub fn edge_left(&self) -> Line {
        Line {
            v0: self.top_left(),
            v1: self.bottom_left(),
        }
    }

    pub fn edge_right(&self) -> Line {
        Line {
            v0: self.top_right(),
            v1: self.bottom_right(),
        }
    }
}
