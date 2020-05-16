use super::Brick;
use crate::pipelines::{Sprite, SpritePipeline};
use fine::graphic::{wgpu, Gpu, TextureAtlas, AsTextureView};
use fine::math::{Vector2, Vector4};

pub struct Level {
    pub bricks: Vec<Sprite>,
    pub player: Sprite,
    pub bg: Sprite,
    pub texture_binding: wgpu::BindGroup,
}

impl Level {
    pub fn generate(width: u32, height: u32, gpu: &mut Gpu, pipeline: &SpritePipeline) -> Self {
        let img =
            image::load_from_memory(&include_bytes!("../assets/images/game.png")[..]).unwrap();
        let mut atlas = TextureAtlas::new(gpu, 256, 256, 4);
        atlas.append_image(gpu, &img, |atlas| {
            atlas.add("blue", 0, Vector4::new(0, 0, 150, 50), None);
            atlas.add("yellow", 1, Vector4::new(0, 50, 150, 50), None);
            atlas.add("paddle", 2, Vector4::new(0, 100, 200, 48), None);
        });

        let (w, h, ..) = atlas.dimensions();
        let texture_binding =
            pipeline.create_texture_binding(gpu, &atlas, w as f32, h as f32);

        let mut bricks: Vec<Sprite> = (0..width * height)
            .map(|index| {
                let index = index as f32;
                let width = width as f32;
                let height = height as f32;

                let x = index % width;
                let y = f32::floor(index / width);

                let frame = if y % 2f32 == 0f32 { "blue" } else { "yellow" };

                let mut sprite = Sprite::from_atlas(frame, &atlas);
                sprite.transform.scale(0.5);

                let offset_x = 1f32 * x - (width * 0.5 * sprite.width()) + sprite.width() * 0.5;
                let offset_y = 1f32 * y;

                sprite.transform.translate(
                    x * sprite.width() as f32 + offset_x,
                    y * sprite.height() as f32 + offset_y,
                    0.0,
                );

                sprite
            })
            .collect();

        let mut player = Sprite::from_atlas("paddle", &atlas);
        player.transform.scale(0.5);
        player.transform.translate(0.0, -200.0, 0.0);

        let size = 64u32;
        let bg_texture = fine::graphic::create_texture_color(0x0C0F1A, size, size, gpu);
        atlas.append_raw_texture("bg", 3, gpu, &bg_texture, Vector4::new(0, 0, size, size), None);

        let mut bg = Sprite::from_atlas("bg", &atlas);
        bg.transform.scale(100.0);

        Self {
            bricks,
            player,
            texture_binding,
            bg,
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
            .chain(self.bricks.iter())
            .chain(std::iter::once(&self.player));

        pipeline.draw(gpu, attachment, camera, &self.texture_binding, it);
    }
}
