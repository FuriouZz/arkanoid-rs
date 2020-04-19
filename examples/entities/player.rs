use fine::wasm::canvas;

pub struct Player {
  pub x: f64,
  pub y: f64,
  pub width: f64,
  pub height: f64,
  _x: f64,
  _y: f64,
}

impl Player {

  pub fn new() -> Self {
    Self {
      x: 0.,
      y: 0.,
      _x: 0.,
      _y: 0.,
      width: 150.,
      height: 15.,
    }
  }

  pub fn position(&mut self, x: f64, y: f64) {
    self._x = x;
    self.x = x;
    self._y = y;
    self.y = y;
  }

  pub fn update(&mut self) {
    self._x += (self.x - self._x) * 0.15;
    self._y += (self.y - self._y) * 0.15;
  }

  pub fn draw(&self) {
    canvas::fill_style("red");
    canvas::fill_rect(self._x - self.width * 0.5, self._y, self.width, self.height);
  }

}