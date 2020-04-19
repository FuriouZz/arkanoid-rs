use crate::{common::Drawable, GameState};
use fine::{event::KeyCode, math::Vec2, wasm::canvas};

pub struct Player {
    pub position: Vec2,
    pub size: Vec2,
    _position: Vec2,
}

impl Player {
    pub fn new() -> Self {
        let position = Vec2::new();
        let _position = Vec2::new();
        let mut size = Vec2::new();
        size.x = 150.;
        size.y = 25.;

        Self {
            _position,
            position,
            size,
        }
    }

    pub fn reset(&mut self, x: f64, y: f64) {
        self._position.x = x;
        self.position.x = x;
        self._position.y = y;
        self.position.y = y;
    }
}

impl Drawable for Player {
    fn resize(&mut self, s: &GameState) {
        self.reset(s.screen.0 * 0.5, s.screen.1 - self.size.y)
    }

    fn update(&mut self, s: &GameState) {
        if s.pressed.contains(&KeyCode::Left) {
            self.position.x -= 1. * s.dt;
        }
        if s.pressed.contains(&KeyCode::Right) {
            self.position.x += 1. * s.dt;
        }

        if self.position.x - self.size.x * 0.5 < 0. {
            self.position.x = self.size.x * 0.5;
        }
        if self.position.y < self.size.y * 0.5 {
            self.position.y = self.size.y * 0.5;
        }
        if self.position.x + self.size.x * 0.5 > s.screen.0 {
            self.position.x = s.screen.0 - self.size.x * 0.5;
        }
        if self.position.y + self.size.y * 0.5 > s.screen.1 {
            self.position.y = s.screen.1 - self.size.y * 0.5;
        }

        self._position.x += (self.position.x - self._position.x) * 0.125;
        self._position.y += (self.position.y - self._position.y) * 0.125;
    }

    fn draw(&self) {
        canvas::fill_style_static("blue");
        canvas::fill_rect(
            self._position.x - self.size.x * 0.5,
            self._position.y,
            self.size.x,
            self.size.y,
        );
    }
}
