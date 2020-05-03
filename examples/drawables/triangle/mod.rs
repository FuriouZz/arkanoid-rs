use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

fn create_triangle(device: &wgpu::Device) -> Triangle {
    let vertices: Vec<Vertex> = vec![
        vertex(0.0, 1.0, 0., 0.5, 1.0),
        vertex(1.0, -1.0, 0., 1.0, 0.0),
        vertex(-1.0, -1.0, 0., 0.0, 0.0),
    ];

    let indices: Vec<u16> = vec![0, 1, 2];

    let vertex_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&vertices),
        wgpu::BufferUsage::VERTEX,
    );

    let index_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&indices),
        wgpu::BufferUsage::INDEX,
    );

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        bindings: &[],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        bindings: &[],
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let source = include_bytes!("./triangle.vert.spv");
    let vertex_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&source[..])).unwrap());
    let source = include_bytes!("./triangle.frag.spv");
    let fragment_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&source[..])).unwrap());

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
            format: crate::DEFAULT_TEXTURE_FORMAT,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        vertex_state: wgpu::VertexStateDescriptor {
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[wgpu::VertexBufferDescriptor {
                stride: Vertex::STRIDE,
                attributes: Vertex::ATTRIBUTES,
                step_mode: wgpu::InputStepMode::Vertex,
            }],
        },
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    Triangle {
        pipeline,
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
        bind_group,
    }
}

pub struct Triangle {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Triangle {
    pub fn new(device: &wgpu::Device) -> Self {
        create_triangle(device)
    }

    pub fn draw(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView) {
        let mut pass = encoder.begin_render_pass(&fine::graphic::wgpu::RenderPassDescriptor {
            color_attachments: &[fine::graphic::wgpu::RenderPassColorAttachmentDescriptor {
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
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_index_buffer(&self.index_buffer, 0, 0);
        pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
