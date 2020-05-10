mod camera;
mod entities;
mod pipelines;
use camera::{Camera, Lens};
use pipelines::{SpriteInstance, SpritePipeline};

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
        let level = entities::Level::generate(2, 2, gpu, &sprite);

        Self {
            level,
            sprite,
            camera: Camera::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 100.0),
        }
    }

    fn on_start(&mut self, _frame: fine::Frame) {
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
    fn on_draw(&mut self, mut frame: fine::Frame) {
        let frame = &mut frame; // Shadowing frame
        let camera = &self.camera;
        let instances: Vec<&SpriteInstance> = self
            .level
            .bricks
            .iter_mut()
            .map(|brick| brick.update(frame, camera))
            .collect();
        self.sprite.draw(frame, instances.as_slice());
    }
}

fn main() {
    fine::start::<ArkanoidScene>(Default::default());
}
