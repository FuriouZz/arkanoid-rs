#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0., y: 0. }
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        if l > 0. {
            self.x /= l;
            self.y /= l;
        }
    }

    pub fn scale(&mut self, scale: f32) {
        self.x *= scale;
        self.y *= scale;
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn distance(&self, v0: &Vec2) -> f32 {
        let x = self.x - v0.x;
        let y = self.y - v0.y;
        f32::sqrt(x * x + y * y)
    }
}

impl Clone for Vec2 {
    fn clone(&self) -> Self {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
