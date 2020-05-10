use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

pub struct QuadExample {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl fine::Scene for QuadExample {
    fn on_load(mut frame: fine::Frame) -> Self
    where
        Self: Sized,
    {
        fine::log!("🚧 QuadExample is loading");

        let gpu = frame.gpu();

        // Let's create vertices
        let vertices: Vec<Vertex> = vec![
            vertex(-1.0, -1.0, 0., 0.0, 0.0),
            vertex(-1.0, 1.0, 0., 0.0, 1.0),
            vertex(1.0, 1.0, 0., 1.0, 1.0),
            vertex(1.0, -1.0, 0., 1.0, 0.0),
        ];

        // Let's create indices
        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        // Create buffer for vertices
        let vertex_buffer = gpu.device.create_buffer_with_data(
            fine::bytemuck::cast_slice(&vertices),
            wgpu::BufferUsage::VERTEX,
        );

        // Create buffer for indices
        let index_buffer = gpu.device.create_buffer_with_data(
            fine::bytemuck::cast_slice(&indices),
            wgpu::BufferUsage::INDEX,
        );

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[],
            });

        let vertex_module = gpu.create_shader_module(&include_bytes!("./quad.vert.spv")[..]);
        let fragment_module = gpu.create_shader_module(&include_bytes!("./quad.frag.spv")[..]);

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
                    // Describe our vertex buffer (attributes, stride, vertex/instance...)
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
        }
    }

    fn on_start(&mut self, _frame: &mut fine::Frame) {
        fine::log!("QuadExample initialized 🥰");
    }

    fn on_draw(&mut self, frame: &mut fine::Frame) {
        let (gpu, view) = frame.target();
        let encoder = &mut gpu.encoder;

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
        // Set index buffer
        pass.set_index_buffer(&self.index_buffer, 0, 0);
        // Set vertex buffer
        pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        // Draw with indices
        pass.draw_indexed(0..self.index_count, 0, 0..1);

        // >> This is hidden by my framework, but here, the encoder is submitted to the render queue!
    }
}

fn main() {
    fine::start::<QuadExample>(Default::default());
}
