use super::Player;
use crate::GameState;
use fine::{
    math::{Circle, Vec2},
    wasm::canvas,
};

pub struct Ball {
    pub collider: Circle,
    pub position: Vec2,
    pub acc: Vec2,
    pub radius: f64,
    pub released: bool,
}

impl Ball {
    pub fn new() -> Self {
        let position = Vec2::new();
        let mut acc = Vec2::new();
        acc.x = 0.;
        acc.y = 0.;

        Self {
            collider: Circle::new(),
            position,
            acc,
            radius: 10.,
            released: false,
        }
    }

    pub fn reset(&mut self) {
        self.released = false;
    }

    pub fn resize(&mut self, s: &GameState) {
        self.reset();
    }

    pub fn update(&mut self, s: &GameState, player: &Player) {
        if s.sticky && !self.released {
            self.stick_player(&player);
        } else if !s.sticky && !self.released {
            self.release();
        } else {
            self.update_position(s);
            self.collide_player(&player);
        }
    }

    fn stick_player(&mut self, player: &Player) {
        let x = player.collider.position.x + player.size.x * 0.5;
        let y = player.collider.position.y - self.radius;
        self.position.x = x;
        self.position.y = y;
    }

    fn collide_player(&mut self, player: &Player) {
        if self.collider.position.y > player.collider.position.y
            && self.collider.position.x - self.collider.radius > player.collider.position.x
            && self.collider.position.x + self.collider.radius
                < player.collider.position.x + player.collider.size.x
        {
            let mut x = self.collider.position.x;
            x -= player.collider.position.x;
            x = x / player.collider.size.x;

            let range = std::f64::consts::PI * 0.5; // 90deg
            let angle = f64::sin(x * std::f64::consts::PI * 0.5) * range;
            let offset = std::f64::consts::PI * -0.5 - range * 0.5;
            self.acc.x = 1. * f64::cos(offset + angle);
            self.acc.y = 1. * f64::sin(offset + angle);
        }
    }

    fn update_position(&mut self, s: &GameState) {
        self.position.x += self.acc.x * s.dt;
        self.position.y += self.acc.y * s.dt;

        self.collider.position.x = self.position.x.round();
        self.collider.position.y = self.position.y.round();
        self.collider.radius = self.radius.round();

        if self.collider.position.x + self.collider.radius > s.screen.size.x
            || self.collider.position.x - self.collider.radius <= 0.
        {
            self.acc.x *= -1.;
        }

        if self.collider.position.y + self.collider.radius > s.screen.size.y
            || self.collider.position.y - self.collider.radius < 0.
        {
            self.acc.y *= -1.;
        }
    }

    fn release(&mut self) {
        self.released = true;
        let range = std::f64::consts::PI * 0.25; // 45deg
        let angle = fine::wasm::rand() * range;
        let offset = std::f64::consts::PI * -0.5 - range * 0.5;
        self.acc.x = 1. * f64::cos(offset + angle);
        self.acc.y = 1. * f64::sin(offset + angle);
    }

    pub fn draw(&self) {
        canvas::fill_style_static("green");
        canvas::begin_path();
        canvas::circle(self.position.x, self.position.y, self.radius);
        canvas::close_path();
        canvas::fill();
    }

    pub fn debug(&self) {
        canvas::fill_style_static("rgba(255, 0, 0, 0.25");
        canvas::stroke_style_static("red");
        canvas::begin_path();
        canvas::circle(self.collider.position.x, self.collider.position.y, self.collider.radius);
        canvas::close_path();
        canvas::fill();
    }
}
