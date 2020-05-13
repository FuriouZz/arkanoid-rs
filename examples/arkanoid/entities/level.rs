use super::Brick;
use crate::pipelines::SpritePipeline;
use fine::graphic::{wgpu, Gpu, TexturePacked};
use fine::math::Vector4;

pub struct Level {
    pub bricks: Vec<Brick>,
    pub texture_binding: wgpu::BindGroup,
}

impl Level {
    pub fn generate(width: u32, height: u32, gpu: &mut Gpu, sprite: &SpritePipeline) -> Self {
        let img = image::load_from_memory(&include_bytes!("../assets/brick3.png")[..]).unwrap();
        let atlas = TexturePacked::from_image(&img)
            .add_rect(Vector4::new(0, 0, 128, 43))
            .add_rect(Vector4::new(0, 43, 128, 43))
            .into_atlas(gpu);

        let texture_binding = sprite.create_texture_binding(gpu, atlas.as_view(), atlas.width(), atlas.height());

        let bricks: Vec<Brick> = (0..width * height)
            .map(|index| {
                let x = (index % width) as f32;
                let y = f32::floor((index as f32) / (width as f32));
                let mut brick = Brick::new(&atlas);
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
