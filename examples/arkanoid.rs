mod entities;
use entities::*;

use fine::{
    start,
    event::{KeyCode, EventHandler},
    wasm::{canvas, console},
};


struct Stage {
    width: f64,
    height: f64,
    player: Option<Player>,
    ball: Option<Ball>,
}

impl Stage {}

impl EventHandler for Stage {

    fn init(&mut self) {
        self.player = Some(Player::new());
        self.ball = Some(Ball::new());
    }

    fn frame(&mut self) {
        canvas::clear();

        if let Some(ref mut player) = self.player {
            player.update();
            player.draw();
        }

        if let Some(ref mut ball) = self.ball {
            ball.update(self.width, self.height);
            ball.draw();
        }
    }

    fn resize(&mut self, width: i32, height: i32) {
        self.width = width as f64;
        self.height = height as f64;
        console::log(format!("Resolution {}x{} from Stage", width, height).as_str());

        if let Some(ref mut player) = self.player {
            player.position(self.width * 0.5, self.height - player.height);
        }

        if let Some(ref mut ball) = self.ball {
            ball.position(self.width * 0.5, self.height * 0.5);
        }
    }

    fn key_pressed(&mut self, keycode: KeyCode) {
        if let Some(ref mut player) = self.player {
            match keycode {
                KeyCode::Left => player.x -= 20.,
                KeyCode::Right => player.x += 20.,
                _ => {}
            }
        }
    }
}

pub fn main() {
    start(Stage {
        width: 0.,
        height: 0.,
        player: None,
        ball: None,
    });
}
