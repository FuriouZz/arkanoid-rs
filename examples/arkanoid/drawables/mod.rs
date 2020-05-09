mod sprite;
pub use sprite::*;

pub trait Drawable {
    fn create_pipeline(&mut self, frame: &mut fine::Frame);
    fn render_pipeline(&mut self, frame: &mut fine::Frame);
}
