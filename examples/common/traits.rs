use crate::GameState;

pub trait Drawable {
  fn resize(&mut self, _state: &GameState) {}
  fn update(&mut self, _state: &GameState) {}
  fn draw(&self);
}