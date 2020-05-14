use crate::math::{Isometry3, Matrix4, Similarity3, Quaternion, UnitQuaternion, Vector3, Rotation3, Translation3};

pub struct Transform {
    i: Isometry3<f32>,
    scaling: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            i: Isometry3::identity(),
            scaling: Vector3::identity(),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.translate_with(&Vector3::new(x, y, z));
    }

    pub fn translate_with(&mut self, v: &Vector3<f32>) {
        self.i.translation.vector = self.i.translation.vector + v;
    }

    pub fn scale(&mut self, s: f32) {
        self.scaling.fill(s);
    }

    pub fn non_uniform_scale(&mut self, x: f32, y: f32, z: f32) {
        self.scaling[0] = x;
        self.scaling[1] = y;
        self.scaling[2] = z;
    }

    pub fn rotate_x(&mut self, rad: f32) {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), rad);
    }

    pub fn rotate_y(&mut self, rad: f32) {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rad);
    }

    pub fn rotate_z(&mut self, rad: f32) {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), rad);
    }

    pub fn translation(&self) -> &Vector3<f32> {
        &self.i.translation.vector
    }

    pub fn translation_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.i.translation.vector
    }

    pub fn scaling(&self) -> &Vector3<f32> {
        &self.scaling
    }

    pub fn scaling_mut(&mut self) -> &mut Vector3<f32> {
        &mut self.scaling
    }

    pub fn rotation(&self) -> &Quaternion<f32> {
        self.i.rotation.quaternion()
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.i.to_homogeneous()
            * Matrix4::<f32>::identity().append_nonuniform_scaling(&self.scaling)
    }

    pub fn decompose(&self) -> (Vector3<f32>, UnitQuaternion<f32>, Vector3<f32>) {
        (
            self.i.translation.vector.clone(),
            self.i.rotation,
            self.scaling.clone()
        )
    }

    pub fn compose(&mut self, translation: Vector3<f32>, rotation: UnitQuaternion<f32>, scaling: Vector3<f32>) {
        self.i.rotation = rotation;
        self.i.translation = Translation3::from(translation);
        self.scaling = scaling;
    }
}
