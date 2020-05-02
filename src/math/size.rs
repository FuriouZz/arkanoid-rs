#[derive(Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new() -> Self {
        Self {
            width: 0.,
            height: 0.,
        }
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        if l > 0. {
            self.width /= l;
            self.height /= l;
        }
    }

    pub fn scale(&mut self, scale: f32) {
        self.width *= scale;
        self.height *= scale;
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.width * self.width + self.height * self.height)
    }

    pub fn distance(&self, v0: &Size) -> f32 {
        let x = self.width - v0.width;
        let y = self.height - v0.height;
        f32::sqrt(x * x + y * y)
    }
}

impl Clone for Size {
    fn clone(&self) -> Self {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}
