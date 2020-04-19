use fine::math::Rect;

pub trait Drawable {
  fn draw(&self);
}

pub trait Debuggable {
  fn debug(&self) -> Rect;
}