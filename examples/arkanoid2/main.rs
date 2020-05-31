mod camera;
mod pipelines;
use camera::{Camera, Lens};
use pipelines::{Sprite, SpritePipeline};

pub struct Arkanoid {
    pipeline: SpritePipeline,
    background: Sprite,
    bricks: Vec<Sprite>,
    camera: Camera,
    texture_binding: fine::graphic::wgpu::BindGroup,
}

impl fine::Scene for Arkanoid {
    fn on_load(mut frame: fine::Frame) -> Self
    where
        Self: Sized,
    {
        let pipeline = SpritePipeline::new(&mut frame);

        // Create texture atlas
        let gpu = frame.gpu();
        let mut atlas = fine::graphic::TextureAtlas::new(gpu, 64, 64, 2);

        // Create background
        let bg_size = 64u32;
        let bg_texture = fine::graphic::create_texture_color(0x1A6CC3, bg_size, bg_size, gpu);
        atlas.append_raw_texture(
            "background",
            0,
            gpu,
            &bg_texture,
            fine::math::Vector4::new(0, 0, bg_size, bg_size),
            None,
        );
        let background = Sprite::from_atlas("background", &atlas);

        // Create bricks
        let brick_size = 64u32;
        let brick_texture =
            fine::graphic::create_texture_color(0xffffff, brick_size, brick_size, gpu);
        atlas.append_raw_texture(
            "brick",
            1,
            gpu,
            &brick_texture,
            fine::math::Vector4::new(0, 0, brick_size, brick_size),
            None,
        );

        let width = 8;
        let height = 8;
        let gap = 10.0;
        let bricks: Vec<Sprite> = (0..width * height)
            .map(|_index| {
                let (layer, rect) = atlas.frame("brick").expect("msg");
                Sprite::from_frame(
                    layer,
                    fine::math::Vector4::new(rect[0], rect[1], 50.0, 15.0),
                )
            })
            .collect();

        // Create binding
        let (w, h, ..) = atlas.dimensions();
        let texture_binding = pipeline.create_texture_binding(gpu, &atlas, w as f32, h as f32);

        // Create orthographic camera
        let camera = Camera::orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 100.0);

        Self {
            pipeline,
            background,
            bricks,
            camera,
            texture_binding,
        }
    }

    fn on_start(&mut self, _frame: &mut fine::Frame) {
        fine::log!("Arkanoid initialized 🥰");
    }

    fn on_event(&mut self, frame: &mut fine::Frame, e: fine::event::Event) {
        match e {
            fine::event::Event::Resize(width, height) => {
                fine::log!("Resolution {}x{}", width, height);
                let width = width as f32;
                let height = height as f32;

                // Update orthographic projection
                let lens = &mut self.camera.lens;
                match lens {
                    Lens::Orthographic(o) => {
                        o.set_left_and_right(0.0, width);
                        o.set_bottom_and_top(0.0, height);
                    }
                    _ => {}
                }

                // Resize depth map
                self.pipeline.resize(frame);

                // Update background size
                self.background
                    .transform
                    .non_uniform_scale(width / 64.0, height / 64.0, 1.0);

                self.background
                    .transform
                    .translate(width * 0.5, height * 0.5, 0.0);

                // Update bricks
                let row = 8.0;
                let gap = 10.0;
                for (index, sprite) in self.bricks.iter_mut().enumerate() {
                    let index = index as f32;

                    let x = index % row;
                    let y = f32::floor(index / row);

                    let mut offset_x = gap * x - (sprite.width() + gap) * (row * 0.5) + (sprite.width() + gap) * 0.5;
                    let mut offset_y = gap * y;

                    offset_x = offset_x + width * 0.5;
                    offset_y = offset_y + width * 0.5;

                    sprite.transform.translate(
                        x * sprite.width() + offset_x,
                        y * sprite.height() + offset_y,
                        0.0,
                    );
                }
            }
            _ => {}
        }
    }

    fn on_draw(&mut self, frame: &mut fine::Frame) {
        let (gpu, attachment) = frame.target();
        let instances = std::iter::once(&self.background).chain(&self.bricks);
        self.pipeline.draw(
            gpu,
            attachment,
            &self.camera,
            &self.texture_binding,
            instances,
        );
    }
}

fn main() {
    fine::start::<Arkanoid>(Default::default());
}
