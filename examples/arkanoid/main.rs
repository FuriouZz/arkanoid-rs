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
        let gpu = frame.gpu();
        let sprite = SpritePipeline::new(gpu);
        let level = entities::Level::generate(10, 10, gpu, &sprite);

        Self {
            level,
            sprite,
            camera: Camera::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 100.0),
        }
    }

    fn on_start(&mut self, _frame: &mut fine::Frame) {
        fine::log!("Arkanoid initialized 🥰");
    }
    fn on_event(&mut self, e: fine::event::Event) {
        match e {
            fine::event::Event::Resize(width, height) => {
                fine::log!("Resolution {}x{}", width, height);
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
            }
            _ => {}
        }
    }
    fn on_draw(&mut self, frame: &mut fine::Frame) {
        let mut b = self.level.bricks.get_mut(2).unwrap();
        let sprite = &mut b.clip.sprite;
        sprite.transform.scale(f32::cos(fine::now() as f32*0.001).abs().max(0.0001));
        sprite.transform.rotate_z(fine::now() as f32*0.00001);
        sprite.transform.translate(f32::cos(fine::now() as f32*0.001) * 1f32, 0.0, 0.0);

        let instances: Vec<&Sprite> = self
            .level
            .bricks
            .iter_mut()
            .map(|brick| &brick.clip.sprite)
            .collect();

        self.sprite.draw(
            frame,
            &self.camera,
            &self.level.texture_binding,
            instances.as_slice(),
        );
    }
}

fn main() {
    fine::start::<ArkanoidScene>(Default::default());
}
