use fine::graphic::{wgpu, Texture2D, Gpu};
use fine::math::{Matrix4, Vector3};

pub struct SpriteInstance {
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    pub texture: Texture2D,
    origin: Matrix4<f32>,
}

impl SpriteInstance {
    pub fn new(bind_group: wgpu::BindGroup, buffer: wgpu::Buffer, texture: Texture2D) -> Self {
        let origin = Matrix4::new_nonuniform_scaling(&Vector3::new(
            texture.width() as f32,
            texture.height() as f32,
            1.0,
        ));

        let mut s = Self {
            bind_group,
            buffer,
            texture,
            origin,
        };

        s.set_origin(-0.5, 0.5);

        s
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        let x = -x * 2.0 * self.texture.width() as f32;
        let y = -y * 2.0 * self.texture.height() as f32;
        self.origin[12] = x;
        self.origin[13] = y;
    }

    pub(super) fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn update_transform(&mut self, gpu: &mut Gpu, transform: &Matrix4<f32>) {
        let t = transform * self.origin;

        // Update transform
        let transform_buffer = gpu.create_buffer(t.as_slice(), wgpu::BufferUsage::COPY_SRC);

        let encoder = &mut gpu.encoder;
        encoder.copy_buffer_to_buffer(
            &transform_buffer,
            0,
            &self.buffer,
            0,
            std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
        );
    }
}