mod triangle;
pub use triangle::*;
mod quad;
pub use quad::*;
mod sprite;
pub use sprite::*;

pub trait Drawable {
    fn create_pipeline(&mut self, frame: &mut fine::Frame);
    fn render_pipeline(&self, frame: &mut fine::Frame);
}
