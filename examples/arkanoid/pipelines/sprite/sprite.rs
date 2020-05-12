use fine::graphic::Texture2DAtlas;
use fine::math::{UnitQuaternion, Vector3, Vector4};

pub struct Sprite {
    layer: u32,
    translation: Vector3<f32>,
    scaling: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    origin: Vector4<f32>,
}

impl Sprite {
    pub fn new(texture: &Texture2DAtlas) -> Self {
        Self {
            layer: 0,
            translation: Vector3::new(0.0, 0.0, 0.0),
            scaling: Vector3::new(1.0, 1.0, 1.0),
            rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            origin: Vector4::new(
                texture.width() as f32 * -0.5,
                texture.height() as f32 * -0.5,
                texture.width() as f32,
                texture.height() as f32,
            ),
        }
    }

    pub fn set_layer(&mut self, layer: u32) {
        self.layer = layer;
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        let x = -x * self.origin[2] as f32;
        let y = -y * self.origin[3] as f32;
        self.origin[0] = x;
        self.origin[1] = y;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.translation[0] = x;
        self.translation[1] = y;
    }

    pub fn x(&self) -> f32 {
        self.translation[0]
    }

    pub fn y(&self) -> f32 {
        self.translation[1]
    }

    pub fn width(&self) -> f32 {
        self.scaling[0] * self.origin[2]
    }

    pub fn height(&self) -> f32 {
        self.scaling[1] * self.origin[3]
    }

    pub(super) fn as_instance(&self) -> super::SpriteInstance {
        super::SpriteInstance {
            layer: self.layer,
            translation: self.translation.clone(),
            rotation: self.rotation.clone(),
            scaling: self.scaling.clone(),
            origin: self.origin.clone(),
        }
    }
}
