use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

// Create texture from buffer
fn create_texture(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) -> wgpu::TextureView {
    let brick = &include_bytes!("../../assets/brick2.png")[..];
    let brick = std::io::Cursor::new(brick);
    let d = png::Decoder::new(brick);
    let (info, mut reader) = d.read_info().expect("cannot read info");
    let png::OutputInfo { width, height, .. } = info;
    let mut buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut buf).expect("cannot read png frame");

    // Create buffer, used for copy
    let buffer = device.create_buffer_with_data(&buf, wgpu::BufferUsage::COPY_SRC);

    // Create texture
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width,
            height,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });

    // Copy buffer to texture
    encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &buffer,
            offset: 0,
            bytes_per_row: 4 * width,
            rows_per_image: 0,
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        wgpu::Extent3d {
            width,
            height,
            depth: 1,
        },
    );

    // Create texture view
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
        format: wgpu::TextureFormat::Rgba8Unorm,
        dimension: wgpu::TextureViewDimension::D2,
        aspect: wgpu::TextureAspect::default(),
        base_mip_level: 0,
        level_count: 1,
        base_array_layer: 0,
        array_layer_count: 1,
    });

    texture_view
}

fn create_quad_pipeline(device: &wgpu::Device) -> (wgpu::RenderPipeline, fine::graphic::Binding) {
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
        .build(device, None);

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[binding.get_layout()],
    });

    let source = &include_bytes!("./quad.vert.spv")[..];
    let vertex_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(source)).unwrap());
    let source = &include_bytes!("./quad.frag.spv")[..];
    let fragment_module =
        device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(source)).unwrap());

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

fn create_quad(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) -> Quad {
    let vertices: Vec<Vertex> = vec![
        vertex(-1.0, -1.0, 0., 0.0, 0.0),
        vertex(-1.0, 1.0, 0., 0.0, 1.0),
        vertex(1.0, 1.0, 0., 1.0, 1.0),
        vertex(1.0, -1.0, 0., 1.0, 0.0),
    ];

    let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

    let vertex_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&vertices),
        wgpu::BufferUsage::VERTEX,
    );

    let index_buffer = device.create_buffer_with_data(
        fine::bytemuck::cast_slice(&indices),
        wgpu::BufferUsage::INDEX,
    );

    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
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

    let texture_view = create_texture(device, encoder);

    let (pipeline, binding) = create_quad_pipeline(device);

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
            _ => None,
        })
        .collect::<Vec<_>>();

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: binding.get_layout(),
        bindings: &resources,
    });

    Quad {
        pipeline,
        vertex_buffer,
        index_buffer,
        index_count: indices.len() as u32,
        bind_group,
    }
}

pub struct Quad {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    bind_group: wgpu::BindGroup,
}

impl Quad {
    pub fn new(device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) -> Self {
        create_quad(device, encoder)
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
