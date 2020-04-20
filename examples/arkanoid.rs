mod entities;
use entities::*;
use std::collections::HashSet;

use fine::{
    event::{EventHandler, KeyCode},
    start,
    math::Rect,
    wasm::canvas,
};

pub struct Stage {
    state: GameState,
    player: Player,
    ball: Ball,
}

impl Stage {
    fn new() -> Self {
        Self {
            state: GameState::new(),
            player: Player::new(),
            ball: Ball::new(),
        }
    }
}

pub struct GameState {
    screen: Rect,
    pressed: HashSet<KeyCode>,
    dt: f64,
    time: f64,
}

impl GameState {
    fn new() -> Self {
        let pressed = HashSet::new();

        let state = Self {
            screen: Rect::new(),
            pressed,
            time: 0.,
            dt: 0.,
        };

        state
    }
}

impl EventHandler for Stage {
    fn frame(&mut self) {
        let time = fine::wasm::now();
        self.state.dt = time - self.state.time;
        self.state.time = time;

        self.player.update(&self.state);
        self.ball.update(&self.state);

        canvas::clear();
        self.player.draw();
        self.ball.draw();
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.state.screen.size.x = width as f64;
        self.state.screen.size.y = height as f64;
        self.player.resize(&self.state);
        self.ball.resize(&self.state);
        fine::log!("Resolution {}x{}", self.state.screen.size.x, self.state.screen.size.y);
    }

    fn focus(&mut self) {
        self.state.time = fine::wasm::now();
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
