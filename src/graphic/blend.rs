pub use wgpu::BlendFactor;
pub use wgpu::BlendOperation;
pub use wgpu::BlendDescriptor;

pub const DEFAULT: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::One,
    dst_factor: BlendFactor::Zero,
    operation: BlendOperation::Add,
};

pub const REPLACE: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::One,
    dst_factor: BlendFactor::Zero,
    operation: BlendOperation::Add,
};

pub const TRANSPARENT: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::One,
    dst_factor: BlendFactor::OneMinusSrcAlpha,
    operation: BlendOperation::Add,
};

pub const NORMAL: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcAlpha,
    dst_factor: BlendFactor::OneMinusSrcAlpha,
    operation: BlendOperation::Add,
};

pub const ADD: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcColor,
    dst_factor: BlendFactor::DstColor,
    operation: BlendOperation::Add,
};

pub const SUBTRACT: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcColor,
    dst_factor: BlendFactor::DstColor,
    operation: BlendOperation::Subtract,
};

pub const REVERSE_SUBTRACT: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcColor,
    dst_factor: BlendFactor::DstColor,
    operation: BlendOperation::ReverseSubtract,
};

pub const DARKEST: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcColor,
    dst_factor: BlendFactor::DstColor,
    operation: BlendOperation::Min,
};

pub const LIGHTEST: BlendDescriptor = BlendDescriptor {
    src_factor: BlendFactor::SrcColor,
    dst_factor: BlendFactor::DstColor,
    operation: BlendOperation::Max,
};