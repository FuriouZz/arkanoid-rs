use nalgebra::{Vector3, Isometry3, UnitQuaternion, Matrix4};

#[derive(Debug)]
pub struct Transform {
    i: Isometry3<f32>,
    scaling: Vector3<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            i: Isometry3::identity(),
            scaling: Vector3::new(1.0, 1.0, 1.0),
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.translate_with(&Vector3::new(x, y, z))
    }

    pub fn translate_with(&mut self, v: &Vector3<f32>) -> &mut Self {
        self.i.translation.vector = self.i.translation.vector + v;
        self
    }

    pub fn scale(&mut self, s: f32) -> &mut Self {
        self.scaling.fill(s);
        self
    }

    pub fn non_uniform_scale(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.scaling[0] = x;
        self.scaling[1] = y;
        self.scaling[2] = z;
        self
    }

    pub fn rotate_x(&mut self, rad: f32) -> &mut Self {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::x_axis(), rad);
        self
    }

    pub fn rotate_y(&mut self, rad: f32) -> &mut Self {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::y_axis(), rad);
        self
    }

    pub fn rotate_z(&mut self, rad: f32) -> &mut Self {
        self.i = self.i * UnitQuaternion::from_axis_angle(&Vector3::z_axis(), rad);
        self
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

    pub fn rotation(&self) -> &UnitQuaternion<f32> {
        &self.i.rotation
    }

    pub fn rotation_mut(&mut self) -> &mut UnitQuaternion<f32> {
        &mut self.i.rotation
    }

    pub fn matrix(&self) -> Matrix4<f32> {
        self.i.to_homogeneous()
            * Matrix4::<f32>::identity().append_nonuniform_scaling(&self.scaling)
    }

    pub fn decompose(&self) -> (Vector3<f32>, UnitQuaternion<f32>, Vector3<f32>) {
        (
            self.i.translation.vector.clone(),
            self.i.rotation,
            self.scaling.clone(),
        )
    }

    pub fn compose(
        &mut self,
        translation: Vector3<f32>,
        rotation: UnitQuaternion<f32>,
        scaling: Vector3<f32>,
    ) {
        self.i.rotation = rotation;
        self.i.translation.vector = translation;
        self.scaling = scaling;
    }
}
