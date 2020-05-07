use crate::drawables::Sprite;

pub struct Brick {
    sprite: Option<Sprite>,
}

impl Brick {
    pub fn new() -> Self {
        Self { sprite: None }
    }
}

impl crate::drawables::Drawable for Brick {
    fn create_pipeline(&mut self, frame: &mut fine::Frame) {
        let gpu = frame.gpu();
        let quad = Sprite::new(gpu);
        self.sprite = Some(quad);
    }

    fn render_pipeline(&self, frame: &mut fine::Frame) {
        self.sprite.as_ref().map(|drawable| {
            drawable.draw(frame);
        });
    }
}
