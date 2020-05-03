
pub trait VertexAttributeDescriptor {
    /// The stride, in bytes, between elements of this buffer.
    const STRIDE: wgpu::BufferAddress;
    /// A description of each of the vertex's attributes.
    const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor];
}

pub mod position {
    use super::VertexAttributeDescriptor;
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Vertex {
        pub position: (f32, f32, f32)
    }

    unsafe impl Pod for Vertex {}
    unsafe impl Zeroable for Vertex {}

    impl VertexAttributeDescriptor for Vertex {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Vertex>() as _;
        const ATTRIBUTES: &'static [wgpu::VertexAttributeDescriptor] =
            &[wgpu::VertexAttributeDescriptor {
                format: wgpu::VertexFormat::Float3,
                offset: 0,
                shader_location: 0,
            }];
    }
}

pub mod position_texcoord {
    use super::VertexAttributeDescriptor;
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Vertex {
        pub position: (f32, f32, f32),
        pub texcoord: (f32, f32)
    }

    unsafe impl Pod for Vertex {}
    unsafe impl Zeroable for Vertex {}

    impl VertexAttributeDescriptor for Vertex {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Vertex>() as _;
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
}

pub mod position_texcoord_normal {
    use super::VertexAttributeDescriptor;
    use bytemuck::{Pod, Zeroable};

    #[repr(C)]
    #[derive(Clone, Copy, Debug)]
    pub struct Vertex {
        pub position: (f32, f32, f32),
        pub texcoord: (f32, f32),
        pub normal: (f32, f32, f32),
    }

    unsafe impl Pod for Vertex {}
    unsafe impl Zeroable for Vertex {}

    impl VertexAttributeDescriptor for Vertex {
        const STRIDE: wgpu::BufferAddress = std::mem::size_of::<Vertex>() as _;
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
