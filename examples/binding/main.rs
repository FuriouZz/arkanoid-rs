use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;
use fine::math::Matrix4;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

pub struct BindingExample {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    transform: Matrix4<f32>,
    transform_buffer: wgpu::Buffer,
    transform_bind_group: wgpu::BindGroup,
}

impl fine::Scene for BindingExample {
    fn on_load(mut frame: fine::Frame) -> Self
    where
        Self: Sized,
    {
        fine::log!("🚧 BindingExample is loading");

        let gpu = frame.gpu();

        let vertices: Vec<Vertex> = vec![
            vertex(-1.0, -1.0, 0., 0.0, 0.0),
            vertex(-1.0, 1.0, 0., 0.0, 1.0),
            vertex(1.0, 1.0, 0., 1.0, 1.0),
            vertex(1.0, -1.0, 0., 1.0, 0.0),
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        let vertex_buffer = gpu.device.create_buffer_with_data(
            fine::bytemuck::cast_slice(&vertices),
            wgpu::BufferUsage::VERTEX,
        );

        let index_buffer = gpu.device.create_buffer_with_data(
            fine::bytemuck::cast_slice(&indices),
            wgpu::BufferUsage::INDEX,
        );

        // Create a buffer from Matrix4<f32>
        let transform = Matrix4::<f32>::identity();
        let transform_buffer = gpu.create_buffer(transform.as_slice(), wgpu::BufferUsage::COPY_DST | wgpu::BufferUsage::UNIFORM);

        // Describe bind group with a layout
        let transform_binding = fine::graphic::BindingDescriptor::new()
            .entry(wgpu::BindGroupLayoutEntry {
                // Define binding index
                binding: 0,
                // Define binding visibility
                visibility: wgpu::ShaderStage::VERTEX,
                // Define binding type (UniformBuffer, StorageBuffer, Sampler, SampledTexture, StorageTexture...)
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            })
            .build(&gpu.device);

        // Create bind group with layout and resources
        let transform_bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("[fine] transform bind group"),
            layout: transform_binding.get_layout(),
            bindings: &[wgpu::Binding {
                // Set binding index
                binding: 0,
                // Set binding resource (Buffer, Sampler, TextureView...)
                resource: wgpu::BindingResource::Buffer {
                    buffer: &transform_buffer,
                    range: 0..std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
                },
            }],
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                // Do not forget to set the bind group layout to the pipeline layout!
                bind_group_layouts: &[transform_binding.get_layout()],
            });

        let vertex_module = gpu.create_shader_module(&include_bytes!("./binding.vert.spv")[..]);
        let fragment_module = gpu.create_shader_module(&include_bytes!("./binding.frag.spv")[..]);

        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vertex_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fragment_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Front,
                    depth_bias: 0,
                    depth_bias_clamp: 0.,
                    depth_bias_slope_scale: 0.,
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor {
                    format: fine::graphic::DEFAULT_TEXTURE_FORMAT,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[wgpu::VertexBufferDescriptor {
                        attributes: Vertex::ATTRIBUTES,
                        stride: Vertex::STRIDE,
                        step_mode: wgpu::InputStepMode::Vertex,
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            transform,
            transform_buffer,
            transform_bind_group,
        }
    }

    fn on_start(&mut self, _frame: fine::Frame) {
        fine::log!("BindingExample initialized 🥰");
    }

    fn on_draw(&mut self, mut frame: fine::Frame) {
        let (gpu, view) = frame.target();

        // Let's move!
        let time = (fine::now() * 0.001) as f32;
        self.transform[12] = f32::cos(time);

        let copy = gpu.create_buffer(self.transform.as_slice(), wgpu::BufferUsage::COPY_SRC);

        // Commit transformation update to the encoder
        let encoder = &mut gpu.encoder;
        encoder.copy_buffer_to_buffer(
            &copy,
            0,
            &self.transform_buffer,
            0,
            std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
        );

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 1.,
                },
            }],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(&self.pipeline);
        // Set bind group to the render pass
        pass.set_bind_group(0, &self.transform_bind_group, &[]);
        pass.set_index_buffer(&self.index_buffer, 0, 0);
        pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        pass.draw_indexed(0..self.index_count, 0, 0..1);

        // >> This is hidden by my framework, but here, the encoder is submitted to the render queue!
    }
}

fn main() {
    fine::start::<BindingExample>(Default::default());
}
