use fine::graphic::{Texture, TextureAtlas};
use fine::math::{UnitQuaternion, Vector2, Vector3, Vector4};
use super::SpriteInstance;

pub struct Sprite {
    layer: u32,
    translation: Vector3<f32>,
    scaling: Vector3<f32>,
    rotation: UnitQuaternion<f32>,
    origin: Vector2<f32>,
    layer_rect: Vector4<f32>,
}

impl Sprite {
    pub fn from_frame(layer: u32, frame: &Vector4<f32>) -> Self {
        Self {
            layer,
            translation: Vector3::new(0.0, 0.0, 0.0),
            scaling: Vector3::new(1.0, 1.0, 1.0),
            rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            origin: Vector2::new(
                0.5,
                0.5,
            ),
            layer_rect: frame.clone(),
        }
    }

    pub fn from_atlas(name: &str, atlas: &TextureAtlas) -> Self {
        let (layer, frame) = atlas.frame(name);
        Self::from_frame(*layer, frame)
    }

    pub fn set_frame(&mut self, layer: u32, frame: &Vector4<f32>) {
        self.layer = layer;
        self.layer_rect = frame.clone();
    }

    pub fn set_frame_from_atlas(&mut self, name: &str, atlas: &TextureAtlas) {
        let (layer, frame) = atlas.frame(name);
        self.set_frame(*layer, frame);
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.origin[0] = x;
        self.origin[1] = y;
    }

    pub fn set_position(&mut self, x: f32, y: f32) {
        self.translation[0] = x;
        self.translation[1] = y;
    }

    pub fn scale(&mut self, s: f32) {
        self.scaling[0] = s;
        self.scaling[1] = s;
    }

    pub fn rotate(&mut self, rad: f32) {
        self.rotation = UnitQuaternion::from_euler_angles(0.0, 0.0, rad);
    }

    pub fn x(&self) -> f32 {
        self.translation[0]
    }

    pub fn y(&self) -> f32 {
        self.translation[1]
    }

    pub fn width(&self) -> f32 {
        self.scaling[0] * self.layer_rect[2]
    }

    pub fn height(&self) -> f32 {
        self.scaling[1] * self.layer_rect[3]
    }

    pub(super) fn as_instance(&self) -> SpriteInstance {
        SpriteInstance {
            layer: Vector3::new(self.layer as f32, self.origin[0], self.origin[1]),
            layer_rect: self.layer_rect.clone(),
            translation: self.translation.clone(),
            scaling: self.scaling.clone(),
            rotation: self.rotation.clone(),
        }
    }
}
