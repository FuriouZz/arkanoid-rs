use fine::wasm::canvas;

pub struct Ball {
  pub x: f64,
  pub y: f64,
  pub acc_x: f64,
  pub acc_y: f64,
  pub radius: f64,
}

impl Ball {

  pub fn new() -> Self {
    Self {
      x: 0.,
      y: 0.,
      acc_x: 10.,
      acc_y: 10.,
      radius: 10.,
    }
  }

  pub fn position(&mut self, x: f64, y: f64) {
    self.x = x;
    self.y = y;
  }

  pub fn update(&mut self, width: f64, height: f64) {
    if self.x >= width || self.x <= 0. {
      self.acc_x *= -1.;
    }
    if self.y >= height || self.y <= 0. {
      self.acc_y *= -1.;
    }

    self.x += self.acc_x;
    self.y += self.acc_y;
  }

  pub fn draw(&self) {
    canvas::fill_style("green");
    canvas::begin_path();
    canvas::circle(self.x, self.y, self.radius);
    canvas::close_path();
    canvas::fill();
  }

}