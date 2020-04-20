use crate::GameState;
use fine::{math::Vec2, wasm::canvas};

pub struct Ball {
    pub position: Vec2,
    pub acc: Vec2,
    pub radius: f64,
}

impl Ball {
    pub fn new() -> Self {
        let position = Vec2::new();
        let mut acc = Vec2::new();
        acc.x = 500.;
        acc.y = 500.;

        Self {
            position,
            acc,
            radius: 10.,
        }
    }

    pub fn reset(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
    }

    pub fn resize(&mut self, s: &GameState) {
        self.reset(s.screen.size.x * 0.5, s.screen.size.y * 0.5);
    }

    pub fn update(&mut self, s: &GameState) {
        self.position.x += self.acc.x * s.dt;
        self.position.y += self.acc.y * s.dt;

        if self.position.x > s.screen.size.x || self.position.x < 0. {
            self.acc.x *= -1.;
        }

        if self.position.y > s.screen.size.y || self.position.y < 0. {
            self.acc.y *= -1.;
        }
    }

    pub fn draw(&self) {
        canvas::fill_style_static("green");
        canvas::begin_path();
        canvas::circle(self.position.x, self.position.y, self.radius);
        canvas::close_path();
        canvas::fill();
    }
}
