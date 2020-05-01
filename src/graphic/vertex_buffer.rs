pub trait VertexAttributeDescriptor {
    /// The stride, in bytes, between elements of this buffer.
    const STRIDE: wgpu::BufferAddress;
    /// A description of each of the vertex's attributes.
    const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor];
}

pub mod defaults {
    use super::VertexAttributeDescriptor;

    pub struct Position {
        position: (f32, f32, f32),
    }

    pub struct PositionTexCoord {
        position_texcoord: (f32, f32, f32, f32, f32),
    }

    pub struct PositionTexCoordNormal {
        position_texcoord_normal: (f32, f32, f32, f32, f32, f32, f32, f32),
    }

    impl VertexAttributeDescriptor for Position {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Position>() as _;
        const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor] =
            &[wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            }];
    }

    impl VertexAttributeDescriptor for PositionTexCoord {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<PositionTexCoord>() as _;
        const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor] = &[
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float2,
                offset: 3 * 4,
                shader_location: 1,
            },
        ];
    }

    impl VertexAttributeDescriptor for PositionTexCoordNormal {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<PositionTexCoordNormal>() as _;
        const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor] = &[
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            },
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float2,
                offset: 3 * 4,
                shader_location: 1,
            },
            wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 3 * 4 + 2 * 4,
                shader_location: 2,
            },
        ];
    }

}