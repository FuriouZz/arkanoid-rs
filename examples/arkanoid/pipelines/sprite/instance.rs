use fine::math::{Matrix4, UnitQuaternion, Vector3, Vector4};

#[derive(Clone, Copy, Debug)]
pub struct Instance {
    pub(super) layer: Vector4<f32>,
    pub(super) layer_rect: Vector4<f32>,
    pub(super) translation: Vector3<f32>,
    pub(super) scaling: Vector3<f32>,
    pub(super) rotation: UnitQuaternion<f32>,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

impl Instance {
    pub const SIZE: u64 = std::mem::size_of::<Instance>() as _;
    pub const MAX: usize = 1_000;

    pub fn as_bytes(&self) -> Vec<u8> {
        bytemuck::cast_slice(&[self.clone()]).to_vec()
    }
}

pub trait AsInstance {
    fn as_instance(&self) -> Instance;
}
