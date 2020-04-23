use crate::GameState;
use fine::{event::KeyCode, math::{Rect, Vec2}, wasm::canvas};

pub struct Player {
    pub collider: Rect,
    pub position: Vec2,
    pub size: Vec2,
    pub late_position: Vec2,
}

impl Player {
    pub fn new() -> Self {
        let position = Vec2::new();
        let _position = Vec2::new();
        let mut size = Vec2::new();
        size.x = 150.;
        size.y = 25.;

        Self {
            collider: Rect::new(),
            late_position: _position,
            position,
            size,
        }
    }

    pub fn reset(&mut self, x: f64, y: f64) {
        self.late_position.x = x;
        self.position.x = x;
        self.late_position.y = y;
        self.position.y = y;
    }

    pub fn resize(&mut self, s: &GameState) {
        self.reset(s.screen.size.x * 0.5, s.screen.size.y - self.size.y);
    }

    pub fn update(&mut self, s: &GameState) {
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
        if self.position.x + self.size.x * 0.5 > s.screen.size.x {
            self.position.x = s.screen.size.x - self.size.x * 0.5;
        }
        if self.position.y + self.size.y * 0.5 > s.screen.size.y {
            self.position.y = s.screen.size.y - self.size.y * 0.5;
        }

        self.late_position.x += (self.position.x - self.late_position.x) * 0.125;
        self.late_position.y += (self.position.y - self.late_position.y) * 0.125;

        self.collider.position.x = (self.late_position.x - self.size.x * 0.5).round();
        self.collider.position.y = (self.late_position.y - 20.).round();
        self.collider.size.x = (self.size.x).round();
        self.collider.size.y = (self.size.y + 20.).round();
    }

    pub fn draw(&self) {
        canvas::fill_style_static("blue");
        canvas::fill_rect(
            self.late_position.x - self.size.x * 0.5,
            self.late_position.y,
            self.size.x,
            self.size.y,
        );
    }

    pub fn debug(&self) {
        canvas::fill_style_static("rgba(255, 0, 0, 0.25");
        canvas::stroke_style_static("red");
        canvas::fill_rect(self.collider.position.x, self.collider.position.y, self.collider.size.x, self.collider.size.y);
        canvas::stroke_rect(self.collider.position.x, self.collider.position.y, self.collider.size.x, self.collider.size.y);
    }

}
