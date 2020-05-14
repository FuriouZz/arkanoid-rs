use super::SpriteInstance;
use fine::graphic::{Texture, TextureAtlas};
use fine::math::{UnitQuaternion, Vector2, Vector3, Vector4};
use fine::Transform;

pub struct Sprite {
    layer: u32,
    layer_rect: Vector4<f32>,
    pub transform: Transform,
    pub origin: Vector2<f32>,
}

impl Sprite {
    pub fn from_frame(layer: u32, frame: &Vector4<f32>) -> Self {
        Self {
            layer,
            transform: Transform::new(),
            origin: Vector2::new(0.5, 0.5),
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

    pub fn x(&self) -> f32 {
        self.transform.translation()[0]
    }

    pub fn y(&self) -> f32 {
        self.transform.translation()[1]
    }

    pub fn width(&self) -> f32 {
        self.transform.scaling()[0] * self.layer_rect[2]
    }

    pub fn height(&self) -> f32 {
        self.transform.scaling()[1] * self.layer_rect[3]
    }

    pub(super) fn as_instance(&self) -> SpriteInstance {
        let (translation, rotation, scaling) = self.transform.decompose();

        SpriteInstance {
            layer: Vector3::new(self.layer as f32, self.origin[0], self.origin[1]),
            layer_rect: self.layer_rect.clone(),
            translation,
            scaling,
            rotation,
        }
    }
}
