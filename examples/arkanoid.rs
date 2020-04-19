mod entities;
mod common;
use entities::*;
use common::{Drawable, Debuggable};

use fine::{
    event::{EventHandler, KeyCode},
    start,
    wasm::canvas, math::Rect,
};

struct Stage {
    width: f64,
    height: f64,
    key_left: bool,
    key_right: bool,
    player: Option<Player>,
    ball: Option<Ball>,
}

impl Stage {
    fn debug(&self, rect: &Rect) {
        canvas::fill_style_static("rgba(255, 0, 0, 0.25)");
        canvas::stroke_style_static("rgba(255, 0, 0, 0.25)");
        canvas::fill_rect(rect.position.x, rect.position.y, rect.size.x, rect.size.y);
        canvas::stroke_rect(rect.position.x, rect.position.y, rect.size.x, rect.size.y);
    }
}

impl EventHandler for Stage {
    fn init(&mut self) {
        let player = Player::new();
        let ball = Ball::new();

        self.player = Some(player);
        self.ball = Some(ball);
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
            let r = player.debug();
            self.debug(&r);
        }

        if let Some(ref mut ball) = self.ball {
            ball.update(self.width, self.height);
            ball.draw();

            let r = ball.debug();
            self.debug(&r);
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
