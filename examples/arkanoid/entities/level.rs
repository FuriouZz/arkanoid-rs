use super::Brick;
use crate::pipelines::{Sprite, SpritePipeline};
use fine::graphic::{wgpu, Gpu, TextureAtlas};
use fine::math::{Vector2, Vector4};
use std::rc::Rc;

pub struct Level {
    pub bricks: Vec<Brick>,
    pub texture_binding: wgpu::BindGroup,
}

impl Level {
    pub fn generate(width: u32, height: u32, gpu: &mut Gpu, sprite: &SpritePipeline) -> Self {
        let img = image::load_from_memory(&include_bytes!("../assets/brick3.png")[..]).unwrap();
        let mut atlas = TextureAtlas::new(gpu, 128, 86, 1);
        atlas.append_image(gpu, &img, |atlas| {
            atlas.add("blue", 0, Vector4::new(0, 0, 128, 43), None);
            atlas.add(
                "green",
                0,
                Vector4::new(0, 43, 128, 43),
                Some(Vector2::new(0, 43)),
            );
        });

        let (w, h, ..) = atlas.dimensions();
        let texture_binding =
            sprite.create_texture_binding(gpu, atlas.as_view(), w as f32, h as f32);

        // let atlas = Rc::new(atlas);

        let bricks: Vec<Brick> = (0..width * height)
            .map(|index| {
                let x = (index % width) as f32;
                let y = f32::floor((index as f32) / (width as f32));

                let frame = if index % 2 == 0 {
                    "green"
                } else {
                    "blue"
                };

                let mut sprite = Sprite::from_atlas("green", &atlas);

                sprite.transform.translate(
                    x * sprite.width() as f32,
                    y * sprite.height() as f32,
                    0.0,
                );

                fine::log!("{}", sprite.width());
                fine::log!("{}", sprite.height());

                Brick {
                    sprite
                }
            })
            .collect();

        Self {
            bricks,
            texture_binding,
        }
    }
}
