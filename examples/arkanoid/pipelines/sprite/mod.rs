use fine::graphic;
use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;
mod clip;
mod sprite;
pub use clip::SpriteClip;
use fine::math::{Matrix4, UnitQuaternion, Vector3, Vector4};
pub use sprite::Sprite;

fn vertex(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
    Vertex {
        position: (x, y, z),
        texcoord: (u, v),
    }
}

pub struct SpritePipeline {
    // Geometry
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
    instance_buffer: wgpu::Buffer,

    // Rendering
    pipeline: wgpu::RenderPipeline,
    constant_group: wgpu::BindGroup,
    instance_layout: graphic::BindingLayout,
    projection_buffer: wgpu::Buffer,
    depth_view: wgpu::TextureView,
}

impl SpritePipeline {
    pub fn new(frame: &mut fine::Frame) -> Self {
        let (frame_width, frame_height) = frame.dimensions();
        let gpu = frame.gpu();

        let vertices: Vec<Vertex> = vec![
            vertex(0.0, 0.0, 0., 0.0, 0.0),
            vertex(0.0, 1.0, 0., 0.0, 1.0),
            vertex(1.0, 1.0, 0., 1.0, 1.0),
            vertex(1.0, 0.0, 0., 1.0, 0.0),
        ];

        let indices: Vec<u16> = vec![0, 1, 2, 0, 2, 3];

        let vertex_buffer = gpu.create_buffer(&vertices, wgpu::BufferUsage::VERTEX);
        let index_buffer = gpu.create_buffer(&indices, wgpu::BufferUsage::INDEX);
        let instance_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            size: SpriteInstance::SIZE * SpriteInstance::MAX as wgpu::BufferAddress,
        });

        let projection_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
        });

        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            compare: wgpu::CompareFunction::Undefined,
            lod_min_clamp: 0.,
            lod_max_clamp: 100.,
        });

        let depth_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: frame_width,
                height: frame_height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        });

        let depth_view = depth_texture.create_default_view();

        let constant_binding = graphic::BindingDescriptor::new()
            // Texture sampler
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
            })
            // Projection matrix
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            })
            .build(&gpu.device);

        let instance_layout = graphic::BindingDescriptor::new()
            // Texture
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    component_type: wgpu::TextureComponentType::Float,
                    dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
            })
            // Atlas size
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            })
            .build(&gpu.device);

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[constant_binding.get_layout(), instance_layout.get_layout()],
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
                    color_blend: graphic::blend::TRANSPARENT,
                    alpha_blend: graphic::blend::TRANSPARENT,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
                depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_read_mask: 0,
                    stencil_write_mask: 0,
                }),
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: Vertex::STRIDE,
                            attributes: Vertex::ATTRIBUTES,
                            step_mode: wgpu::InputStepMode::Vertex,
                        },
                        wgpu::VertexBufferDescriptor {
                            stride: SpriteInstance::SIZE,
                            step_mode: wgpu::InputStepMode::Instance,
                            attributes: &[
                                // Layer and origin
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float3,
                                    offset: 0,
                                    shader_location: 2,
                                },
                                // Layer Rect
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float4,
                                    offset: 3 * 4,
                                    shader_location: 3,
                                },
                                // Translation
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float3,
                                    offset: 3 * 4 + 4 * 4,
                                    shader_location: 4,
                                },
                                // Scaling
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float3,
                                    offset: 3 * 4 + 4 * 4 + 3 * 4,
                                    shader_location: 5,
                                },
                                // Rotation
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float4,
                                    offset: 3 * 4 + 4 * 4 + 3 * 4 + 3 * 4,
                                    shader_location: 6,
                                },
                            ],
                        },
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        let resources = constant_binding.bind(|binding| match binding.binding {
            0 => Some(wgpu::BindingResource::Sampler(&sampler)),
            1 => Some(wgpu::BindingResource::Buffer {
                buffer: &projection_buffer,
                range: 0..std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
            }),
            _ => None,
        });

        let constant_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: constant_binding.get_layout(),
            bindings: resources.as_slice(),
        });

        Self {
            // Geometry
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
            instance_buffer,

            // Rendering
            pipeline,
            constant_group,
            instance_layout,
            projection_buffer,
            depth_view,
        }
    }

    pub fn resize(&mut self, frame: &mut fine::Frame) {
        let (frame_width, frame_height) = frame.dimensions();
        let gpu = frame.gpu();

        let depth_texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: frame_width,
                height: frame_height,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            label: None,
        });

        self.depth_view = depth_texture.create_default_view();
    }

    pub fn create_texture_binding(
        &self,
        gpu: &graphic::Gpu,
        view: &wgpu::TextureView,
        width: f32,
        height: f32,
    ) -> wgpu::BindGroup {
        let buffer = gpu.create_buffer(&[width, height], wgpu::BufferUsage::UNIFORM);

        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: self.instance_layout.get_layout(),
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &buffer,
                        range: 0..std::mem::size_of::<(f32, f32)>() as wgpu::BufferAddress,
                    },
                },
            ],
        })
    }

    pub fn draw(
        &mut self,
        gpu: &mut graphic::Gpu,
        attachment: &wgpu::TextureView,
        camera: &crate::camera::Camera,
        binding: &wgpu::BindGroup,
        instances: &[&Sprite],
    ) {
        // Update projection matrix
        {
            let projection_buffer = gpu.create_buffer(
                camera.get_view_projection().as_slice(),
                wgpu::BufferUsage::COPY_SRC,
            );

            let encoder = &mut gpu.encoder;
            encoder.copy_buffer_to_buffer(
                &projection_buffer,
                0,
                &self.projection_buffer,
                0,
                std::mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
            );
        }

        let mut i = 0;
        let length = instances.len();

        while i < length {
            let offset_start = i;
            let offset_end = (offset_start + SpriteInstance::MAX).min(length);
            let instance_count = offset_end - offset_start;

            let mut bytes: Vec<u8> = Vec::new();

            for j in offset_start..offset_end {
                bytes.extend_from_slice(&instances[j].as_instance().as_bytes());
            }

            let buffer = gpu.create_buffer(&bytes, wgpu::BufferUsage::COPY_SRC);

            let encoder = &mut gpu.encoder;
            encoder.copy_buffer_to_buffer(
                &buffer,
                0,
                &self.instance_buffer,
                0,
                SpriteInstance::SIZE * instance_count as wgpu::BufferAddress,
            );

            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.,
                        g: 0.,
                        b: 0.,
                        a: 1.,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    clear_stencil: 0,
                }),
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.constant_group, &[]);
            pass.set_bind_group(1, binding, &[]);
            pass.set_index_buffer(&self.index_buffer, 0, 0);
            pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
            pass.set_vertex_buffer(1, &self.instance_buffer, 0, 0);
            pass.draw_indexed(0..self.index_count, 0, 0..instance_count as u32);

            i += SpriteInstance::MAX;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SpriteInstance {
    layer: Vector3<f32>,
    layer_rect: Vector4<f32>,
    translation: Vector3<f32>,
    scaling: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
}

unsafe impl bytemuck::Pod for SpriteInstance {}
unsafe impl bytemuck::Zeroable for SpriteInstance {}

impl SpriteInstance {
    const SIZE: u64 = std::mem::size_of::<SpriteInstance>() as _;
    const MAX: usize = 1_000;

    pub fn as_bytes(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[self.clone()]).to_vec()
    }
}
