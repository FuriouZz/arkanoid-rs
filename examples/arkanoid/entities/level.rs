use super::{Ball, Brick, Paddle};
use crate::pipelines::{sprite, Sprite, SpritePipeline};
use fine::graphic::{wgpu, AsTextureView, Gpu, TextureAtlas};
use fine::Frame;
use fine::math::{Vector2, Vector4};
use std::collections::HashSet;

pub struct Level {
    pub bricks: Vec<Brick>,
    pub paddle: Paddle,
    pub bg: Sprite,
    pub ball: Ball,
    pub texture_binding: wgpu::BindGroup,
}

impl Level {
    pub fn generate(width: u32, height: u32, frame: &mut Frame, pipeline: &SpritePipeline) -> Self {
        let (n_widteen_height) = frame.dimensions();
        let gpu = frame.gpu();

        let img =
            image::load_from_memory(&include_bytes!("../assets/images/game.png")[..]).unwrap();
        let mut atlas = TextureAtlas::new(gpu, 256, 256, 5);
        atlas.append_image(gpu, &img, |atlas| {
            atlas.add("blue", 0, Vector4::new(0, 0, 150, 50), None);
            atlas.add("yellow", 1, Vector4::new(0, 50, 150, 50), None);
            atlas.add("paddle", 2, Vector4::new(0, 100, 200, 48), None);
        });

        let bg_size = 64u32;
        let bg_texture = fine::graphic::create_texture_color(0x0C0F1A, bg_size, bg_size, gpu);
        atlas.append_raw_texture(
            "background",
            3,
            gpu,
            &bg_texture,
            Vector4::new(0, 0, bg_size, bg_size),
            None,
        );

        let ball_texture = fine::graphic::create_texture_color(0xffffff, bg_size, bg_size, gpu);
        atlas.append_raw_texture(
            "ball",
            4,
            gpu,
            &ball_texture,
            Vector4::new(0, 0, bg_size, bg_size),
            None,
        );

        let (w, h, ..) = atlas.dimensions();
        let texture_binding = pipeline.create_texture_binding(gpu, &atlas, w as f32, h as f32);

        let mut bricks: Vec<Brick> = (0..width * height)
            .map(|index| {
                let index = index as f32;
                let width = width as f32;
                let height = height as f32;

                let x = index % width;
                let y = f32::floor(index / width);

                let frame = if y % 2f32 == 0f32 { "blue" } else { "yellow" };

                let mut sprite = Sprite::from_atlas(frame, &atlas);
                sprite.transform.scale(0.5);

                let offset_x = 1f32 * x + (width * -0.5 * sprite.width()) + sprite.width() * 0.5
                    - 0.5 * (x + 2.0);
                let offset_y = 1f32 * y;

                sprite.transform.translate(
                    x * sprite.width() + offset_x,
                    y * sprite.height() + offset_y,
                    0.0,
                );

                Brick::new(sprite)
            })
            .collect();

        let mut player = Sprite::from_atlas("paddle", &atlas);
        player.transform.scale(0.5);
        player.transform.translate(0.0, -200.0, 0.0);
        let paddle = Paddle::new(player);

        let mut bg = Sprite::from_atlas("background", &atlas);
        bg.transform
            .non_uniform_scale(1440.0 / bg_size as f32, 754.0 / bg_size as f32, 1.0);

        let mut ball = Sprite::from_atlas("ball", &atlas);
        ball.transform.scale(0.12);
        let ball = Ball::new(ball);

        Self {
            bricks,
            paddle,
            bg,
            ball,
            texture_binding,
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.bg
            .transform
            .non_uniform_scale(width / 64.0, height / 64.0, 1.0);

        self.bg.transform.translate(width * 0.5, height * 0.5, 0.0);
    }

    pub fn update(&mut self, keys: &HashSet<fine::event::KeyCode>, dt: f32) {
        self.paddle.update(keys, dt);
        self.ball.update_with_paddle(&self.paddle);

        if keys.contains(&fine::event::KeyCode::Space) {
            self.ball.release();
        }

        for brick in self.bricks.iter_mut() {
            self.ball.update_with_brick(brick);
        }
    }

    pub fn draw(
        &mut self,
        frame: &mut fine::Frame,
        pipeline: &mut SpritePipeline,
        camera: &crate::Camera,
    ) {
        let (gpu, attachment) = frame.target();

        let it = std::iter::once(&self.bg)
            .chain(self.bricks.iter().filter_map(|b| {
                if b.alive {
                    return Some(&b.sprite);
                }
                return None;
            }))
            .chain(std::iter::once(&self.ball.sprite))
            .chain(std::iter::once(&self.paddle.sprite));

        pipeline.draw(gpu, attachment, camera, &self.texture_binding, it);
    }
}
