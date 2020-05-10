use fine::graphic::{wgpu, Gpu, Texture2D};
use fine::math::{Matrix4, Vector3};

pub struct Sprite {
    bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    texture_width: u32,
    texture_height: u32,
    origin: Matrix4<f32>,
    transform: Matrix4<f32>,
}

impl Sprite {
    pub fn new(gpu: &Gpu, layout: &wgpu::BindGroupLayout, texture: &Texture2D) -> Self {
        let transform_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            usage: wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM,
            size: std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
            label: None,
        });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture.view()),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &transform_buffer,
                        range: 0..std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
                    },
                },
            ],
        });

        let origin = Matrix4::new_nonuniform_scaling(&Vector3::new(
            texture.width() as f32,
            texture.height() as f32,
            1.0,
        ));

        Self {
            bind_group,
            buffer: transform_buffer,
            texture_width: texture.width(),
            texture_height: texture.height(),
            origin,
            transform: Matrix4::identity(),
        }
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        let x = -x * self.texture_width as f32;
        let y = -y * self.texture_height as f32;
        self.origin[12] = x;
        self.origin[13] = y;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.transform[12] = x;
        self.transform[13] = y;
    }

    pub fn x(&self) -> f32 {
        self.transform[12]
    }

    pub fn y(&self) -> f32 {
        self.transform[13]
    }

    pub fn width(&self) -> f32 {
        self.texture_width as f32
    }

    pub fn height(&self) -> f32 {
        self.texture_height as f32
    }

    pub(super) fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub(super) fn update_buffer(&self, gpu: &mut Gpu, projection: &Matrix4<f32>) {
        let t = projection * self.transform * self.origin;

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
