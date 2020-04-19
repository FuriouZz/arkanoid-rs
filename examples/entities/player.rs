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

    pub fn update(&mut self, width: f64, height: f64) {
        if self.x - self.width * 0.5 < 0. {
            self.x = self.width * 0.5;
        }
        if self.y < self.height * 0.5 {
            self.y = self.height * 0.5;
        }
        if self.x + self.width * 0.5 > width {
            self.x = width - self.width * 0.5;
        }
        if self.y + self.height * 0.5 > height {
            self.y = height - self.height * 0.5;
        }

        self._x += (self.x - self._x) * 0.125;
        self._y += (self.y - self._y) * 0.125;
    }

    pub fn draw(&self) {
        canvas::fill_style_static("red");
        canvas::fill_rect(self._x - self.width * 0.5, self._y, self.width, self.height);
    }
}
