mod entities;
use entities::*;

use fine::{
    event::{EventHandler, KeyCode},
    start,
    wasm::canvas,
};

struct Stage {
    width: f64,
    height: f64,
    key_left: bool,
    key_right: bool,
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
            if self.key_left {
                player.x -= 20.;
            }
            if self.key_right {
                player.x += 20.;
            }

            player.update(self.width, self.height);
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
        fine::log!("Resolution {}x{} from Stage", width, height);

        if let Some(ref mut player) = self.player {
            player.position(self.width * 0.5, self.height - player.height);
        }

        if let Some(ref mut ball) = self.ball {
            ball.position(self.width * 0.5, self.height * 0.5);
        }
    }

    fn key_up(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Left => self.key_left = false,
            KeyCode::Right => self.key_right = false,
            _ => {}
        }
    }

    fn key_pressed(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Left => self.key_left = true,
            KeyCode::Right => self.key_right = true,
            _ => {}
        }
    }
}

pub fn main() {
    start(Stage {
        width: 0.,
        height: 0.,
        key_left: false,
        key_right: false,
        player: None,
        ball: None,
    });
}
