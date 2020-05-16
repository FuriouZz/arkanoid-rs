#![allow(unused)]

mod camera;
mod entities;
mod pipelines;
use camera::{Camera, Lens};
use pipelines::{Sprite, SpritePipeline};

pub struct ArkanoidScene {
    camera: Camera,
    sprite: SpritePipeline,
    level: entities::Level,
}

impl fine::Scene for ArkanoidScene {
    fn on_load(mut frame: fine::Frame) -> Self
    where
        Self: Sized,
    {
        let sprite = SpritePipeline::new(&mut frame);
        let gpu = frame.gpu();
        let level = entities::Level::generate(8, 11, gpu, &sprite);

        Self {
            level,
            sprite,
            camera: Camera::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 100.0),
        }
    }

    fn on_start(&mut self, _frame: &mut fine::Frame) {
        fine::log!("Arkanoid initialized 🥰");
    }
    fn on_event(&mut self, frame: &mut fine::Frame, e: fine::event::Event) {
        match e {
            fine::event::Event::Resize(width, height) => {
                fine::log!("Resolution {}x{}", width, height);

                // Update orthographic projection
                let right = width as f32 * 0.5;
                let top = height as f32 * 0.5;
                let lens = &mut self.camera.lens;
                match lens {
                    Lens::Orthographic(o) => {
                        o.set_left_and_right(-right, right);
                        o.set_bottom_and_top(-top, top);
                    }
                    _ => {}
                }

                // Resize depth map
                self.sprite.resize(frame);
            }
            _ => {}
        }
    }
    fn on_draw(&mut self, frame: &mut fine::Frame) {
        self.level.draw(frame, &mut self.sprite, &self.camera);
    }
}

fn main() {
    fine::start::<ArkanoidScene>(Default::default());
}
