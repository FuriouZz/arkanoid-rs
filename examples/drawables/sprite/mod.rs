use cgmath::{prelude::SquareMatrix, Matrix4, Vector3};
use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

fn create_quad_pipeline(
    gpu: &fine::graphic::Gpu,
) -> (wgpu::RenderPipeline, fine::graphic::Binding) {
    let mut binding = fine::graphic::Binding::new();
    binding
        .entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::SampledTexture {
                component_type: wgpu::TextureComponentType::Float,
                dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
        })
        .entry(wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler { comparison: false },
        })
        .entry(wgpu::BindGroupLayoutEntry {
            binding: 2,
            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
        })
        .build(&gpu.device, None);

    let pipeline_layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[binding.get_layout()],
        });

    let vertex_module = gpu.create_shader_module(&include_bytes!("./sprite.vert.spv")[..]);
    let fragment_module = gpu.create_shader_module(&include_bytes!("./sprite.frag.spv")[..]);

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
                    stride: Vertex::STRIDE,
                    attributes: Vertex::ATTRIBUTES,
                    step_mode: wgpu::InputStepMode::Vertex,
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

    (pipeline, binding)
}

fn create_sprite(gpu: &mut fine::graphic::Gpu) -> Sprite {
    let vertices: Vec<Vertex> = vec![
        vertex(-1.0, -1.0, 0., 0.0, 0.0),
        vertex(-1.0, 1.0, 0., 0.0, 1.0),
        vertex(1.0, 1.0, 0., 1.0, 1.0),
        vertex(1.0, -1.0, 0., 1.0, 0.0),
    ];

    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

    let vertex_buffer = gpu.create_buffer(&vertices, wgpu::BufferUsage::VERTEX);
    let index_buffer = gpu.create_buffer(&indices, wgpu::BufferUsage::INDEX);

    let transform = Matrix4::identity();
    let transform_ref: &[f32; 16] = transform.as_ref();
    let transform_buffer = gpu.create_buffer(
        transform_ref,
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );

    let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        compare: wgpu::CompareFunction::Undefined,
        lod_min_clamp: 0.,
        lod_max_clamp: 100.,
    });

    let texture_view = gpu.create_texture_view(&include_bytes!("../../assets/brick2.png")[..]);

    let (pipeline, binding) = create_quad_pipeline(gpu);

    let resources = binding
        .get_entries()
        .filter_map(|b| match b.binding {
            0 => Some(wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            }),
            1 => Some(wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            }),
            2 => Some(wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &transform_buffer,
                    // range: 0..64 // 16 value * 4. Same as:
                    range: 0..std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
                },
            }),
            _ => None,
        })
        .collect::<Vec<_>>();

    let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: binding.get_layout(),
        bindings: &resources,
    });

    Sprite {
        pipeline,
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
        bind_group,
    }
}

pub struct Sprite {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Sprite {
    pub fn new(gpu: &mut fine::graphic::Gpu) -> Self {
        create_sprite(gpu)
    }

    pub fn draw(&self, frame: &mut fine::Frame) {
        let (encoder, attachment) = frame.target();

        let mut pass = encoder.begin_render_pass(&fine::graphic::wgpu::RenderPassDescriptor {
            color_attachments: &[fine::graphic::wgpu::RenderPassColorAttachmentDescriptor {
                attachment,
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
