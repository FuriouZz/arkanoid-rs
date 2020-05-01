pub fn format_size(format: &wgpu::VertexFormat) -> wgpu::BufferAddress {
    match format {
        wgpu::VertexFormat::Char2 => 1 * 2,
        wgpu::VertexFormat::Char4 => 1 * 4,

        wgpu::VertexFormat::Uchar2 => 1 * 2,
        wgpu::VertexFormat::Uchar4 => 1 * 4,

        wgpu::VertexFormat::Char2Norm => 1 * 2,
        wgpu::VertexFormat::Char4Norm => 1 * 4,

        wgpu::VertexFormat::Uchar2Norm => 1 * 2,
        wgpu::VertexFormat::Uchar4Norm => 1 * 4,

        wgpu::VertexFormat::Short2 => 2 * 2,
        wgpu::VertexFormat::Short4 => 2 * 4,

        wgpu::VertexFormat::Ushort2 => 2 * 2,
        wgpu::VertexFormat::Ushort4 => 2 * 4,

        wgpu::VertexFormat::Short2Norm => 2 * 2,
        wgpu::VertexFormat::Short4Norm => 2 * 4,

        wgpu::VertexFormat::Ushort2Norm => 2 * 2,
        wgpu::VertexFormat::Ushort4Norm => 2 * 4,

        wgpu::VertexFormat::Half2 => 2 * 2,
        wgpu::VertexFormat::Half4 => 2 * 4,

        wgpu::VertexFormat::Float => 4,
        wgpu::VertexFormat::Float2 => 4 * 2,
        wgpu::VertexFormat::Float3 => 4 * 3,
        wgpu::VertexFormat::Float4 => 4 * 4,

        wgpu::VertexFormat::Int => 4,
        wgpu::VertexFormat::Int2 => 4 * 2,
        wgpu::VertexFormat::Int3 => 4 * 3,
        wgpu::VertexFormat::Int4 => 4 * 4,

        wgpu::VertexFormat::Uint => 4,
        wgpu::VertexFormat::Uint2 => 4 * 2,
        wgpu::VertexFormat::Uint3 => 4 * 3,
        wgpu::VertexFormat::Uint4 => 4 * 4,
    }
}