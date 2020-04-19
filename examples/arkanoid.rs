mod common;
mod entities;
use common::Drawable;
use entities::*;
use std::collections::HashSet;

use fine::{
    event::{EventHandler, KeyCode},
    start,
    wasm::canvas,
};

pub struct Stage {
    state: GameState,
    drawables: Vec<Box<dyn Drawable>>,
}

impl Stage {
    fn new() -> Self {
        let mut drawables: Vec<Box<dyn Drawable>> = Vec::new();
        drawables.push(Box::new(Player::new()));
        drawables.push(Box::new(Ball::new()));
        Self {
            state: GameState::new(),
            drawables,
        }
    }
}

pub struct GameState {
    screen: (f64, f64),
    pressed: HashSet<KeyCode>,
    dt: f64,
}

impl GameState {
    fn new() -> Self {
        let pressed = HashSet::new();

        let state = Self {
            screen: (0., 0.),
            pressed,
            dt: 0.,
        };

        state
    }
}

impl EventHandler for Stage {
    fn frame(&mut self, dt: f64) {
        self.state.dt = dt;

        for drawable in self.drawables.iter_mut() {
            drawable.update(&self.state);
        }

        canvas::clear();
        for drawable in self.drawables.iter_mut() {
            drawable.draw();
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.state.screen.0 = width as f64;
        self.state.screen.1 = height as f64;

        for drawable in self.drawables.iter_mut() {
            drawable.resize(&self.state);
        }
    }

    fn key_up(&mut self, keycode: KeyCode) {
        self.state.pressed.remove(&keycode);
    }

    fn key_pressed(&mut self, keycode: KeyCode) {
        self.state.pressed.insert(keycode);
    }
}

fn main() {
    start(|| Stage::new());
}