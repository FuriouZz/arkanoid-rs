use fine::graphic;
use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;
use fine::math::{Matrix4, Vector3};

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

pub struct Sprite {
    // Geometry
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,

    // Rendering
    pipeline: wgpu::RenderPipeline,
    constant_group: wgpu::BindGroup,
    texture_layout: graphic::BindingLayout,

    // Transformation
    transform: Matrix4<f32>,
    transform_buffer: wgpu::Buffer,
}

impl Sprite {
    pub fn new(gpu: &mut graphic::Gpu) -> Self {
        let vertices: Vec<Vertex> = vec![
            vertex(-1.0, -1.0, 0., 0.0, 0.0),
            vertex(-1.0, 1.0, 0., 0.0, 1.0),
            vertex(1.0, 1.0, 0., 1.0, 1.0),
            vertex(1.0, -1.0, 0., 1.0, 0.0),
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        let vertex_buffer = gpu.create_buffer(&vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer = gpu.create_buffer(&indices, wgpu::BufferUsage::INDEX);

        let transform = Matrix4::<f32>::identity();
        let transform_ref = transform.as_slice();
        let transform_buffer = gpu.create_buffer(
            transform_ref,
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: wgpu::CompareFunction::Undefined,
            lod_min_clamp: 0.,
            lod_max_clamp: 100.,
        });

        let constant_binding = graphic::BindingDescriptor::new()
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
            })
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            })
            .build(&gpu.device);

        let texture_layout = graphic::BindingDescriptor::from_entry(wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::SampledTexture {
                component_type: wgpu::TextureComponentType::Float,
                dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
        })
        .build(&gpu.device);

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[constant_binding.get_layout(), texture_layout.get_layout()],
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
                    format: graphic::DEFAULT_TEXTURE_FORMAT,
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

        let resources = constant_binding.bind(|b| match b.binding {
            0 => Some(wgpu::BindingResource::Sampler(&sampler)),
            1 => Some(wgpu::BindingResource::Buffer {
                buffer: &transform_buffer,
                range: 0..std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
            }),
            _ => None,
        });

        let constant_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: constant_binding.get_layout(),
            bindings: &resources,
        });

        Self {
            // Geometry
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,

            // Rendering
            pipeline,
            constant_group,
            texture_layout,

            // Transformation
            transform,
            transform_buffer,
        }
    }

    pub fn create_sprite_texture(
        &self,
        gpu: &mut graphic::Gpu,
        texture: graphic::Texture2D,
    ) -> SpriteTexture {
        let binding = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: self.texture_layout.get_layout(),
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            }],
        });
        SpriteTexture(binding, texture)
    }

    pub fn draw(
        &mut self,
        frame: &mut fine::Frame,
        texture: &SpriteTexture,
        transform: &Matrix4<f32>,
    ) {
        let (gpu, attachment) = frame.target();

        // Apply texture size
        self.transform.copy_from(transform);
        self.transform.append_nonuniform_scaling_mut(&Vector3::new(
            texture.1.width() as f32,
            texture.1.height() as f32,
            1.0,
        ));

        // Update transform
        let transform_ref = self.transform.as_slice();
        let transform_buffer = gpu.create_buffer(transform_ref, wgpu::BufferUsage::COPY_SRC);

        let encoder = &mut gpu.encoder;
        encoder.copy_buffer_to_buffer(
            &transform_buffer,
            0,
            &self.transform_buffer,
            0,
            std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
        );

        // Render sprite
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
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
        pass.set_bind_group(0, &self.constant_group, &[]);
        pass.set_bind_group(1, &texture.0, &[]);
        pass.set_index_buffer(&self.index_buffer, 0, 0);
        pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
        pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

pub struct SpriteTexture(pub wgpu::BindGroup, pub fine::graphic::Texture2D);
