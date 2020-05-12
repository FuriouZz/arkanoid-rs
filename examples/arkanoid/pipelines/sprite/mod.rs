use fine::graphic;
use fine::graphic::vertex_attribute::{position_texcoord::Vertex, VertexAttributeDescriptor};
use fine::graphic::wgpu;
mod sprite;
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
}

impl SpritePipeline {
    pub fn new(gpu: &mut graphic::Gpu) -> Self {
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
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: wgpu::CompareFunction::Undefined,
            lod_min_clamp: 0.,
            lod_max_clamp: 100.,
        });

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
            .entry(wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    component_type: wgpu::TextureComponentType::Float,
                    dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: false,
                },
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
                depth_stencil_state: None,
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
                                // Layer
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Uint,
                                    offset: 0,
                                    shader_location: 2,
                                },
                                // Translation
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float3,
                                    offset: 4,
                                    shader_location: 3,
                                },
                                // Scaling
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float3,
                                    offset: 4 + 3 * 4,
                                    shader_location: 4,
                                },
                                // Rotation
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float4,
                                    offset: 4 + 3 * 4 + 3 * 4,
                                    shader_location: 5,
                                },
                                // Origin
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float4,
                                    offset: 4 + 3 * 4 + 3 * 4 + 4 * 4,
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
        }
    }

    pub fn create_sprite(&self, texture: &graphic::Texture2DAtlas) -> Sprite {
        Sprite::new(texture)
    }

    pub fn create_texture_binding(
        &self,
        gpu: &graphic::Gpu,
        texture: &graphic::Texture2DAtlas,
    ) -> wgpu::BindGroup {
        gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: self.instance_layout.get_layout(),
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture.view()),
            }],
        })
    }

    pub fn draw(
        &mut self,
        frame: &mut fine::Frame,
        camera: &crate::camera::Camera,
        texture_binding: &wgpu::BindGroup,
        instances: &[&Sprite],
    ) {
        let (gpu, attachment) = frame.target();

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

        // let instance_count = instances.len();

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
            pass.set_bind_group(1, texture_binding, &[]);
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
    layer: u32,
    translation: Vector3<f32>,
    scaling: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    origin: Vector4<f32>,
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
