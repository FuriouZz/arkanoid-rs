use super::Brick;
use crate::pipelines::SpritePipeline;
use fine::graphic::{wgpu, Gpu, Texture2DAtlas};

pub struct Level {
    pub bricks: Vec<Brick>,
    pub texture_binding: wgpu::BindGroup,
}

impl Level {
    pub fn generate(width: u32, height: u32, gpu: &mut Gpu, sprite: &SpritePipeline) -> Self {
        // let texture = Texture2D::from_image(gpu, &include_bytes!("../assets/brick2.png")[..]);
        // let texture = Texture2D::from_images(gpu, &[
        //     &include_bytes!("../assets/brick2.png")[..],
        //     &include_bytes!("../assets/brick2.png")[..],
        // ]);
        let texture = Texture2DAtlas::from_packed_images(
            gpu,
            128,
            43,
            2,
            &include_bytes!("../assets/brick3.png")[..],
        );
        let texture_binding = sprite.create_texture_binding(gpu, &texture);

        let bricks: Vec<Brick> = (0..width * height)
            .map(|index| {
                let x = (index % width) as f32;
                let y = f32::floor((index as f32) / (width as f32));
                let mut brick = Brick::new(gpu, sprite, &texture);
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
