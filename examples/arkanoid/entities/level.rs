use super::Brick;
use crate::pipelines::SpritePipeline;
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
            atlas.add(
                "blue",
                0,
                Vector4::new(0, 0, 128, 43),
                None,
            );
            atlas.add(
                "green",
                0,
                Vector4::new(0, 43, 128, 43),
                Some(Vector2::new(0, 43)),
            );
        });

        let (w, h, ..) = atlas.dimensions();
        let texture_binding = sprite.create_texture_binding(gpu, atlas.as_view(), w as f32, h as f32);

        let atlas = Rc::new(atlas);

        let bricks: Vec<Brick> = (0..width * height)
            .map(|index| {
                let x = (index % width) as f32;
                let y = f32::floor((index as f32) / (width as f32));
                let mut brick = Brick::new(atlas.clone());

                if index % 2 == 0 {
                    brick.sprite.set_frame("green");
                }

                brick.sprite.set_position(
                    x * brick.sprite.width() as f32,
                    y * brick.sprite.height() as f32,
                );
                brick
            })
            .collect();

        Self {
            bricks,
            texture_binding,
        }
    }
}
