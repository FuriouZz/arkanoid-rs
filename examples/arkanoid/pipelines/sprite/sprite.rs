use super::{AsInstance, Instance};
use fine::graphic::{Texture, TextureAtlas};
use fine::math::{UnitQuaternion, Vector2, Vector3, Vector4};
use fine::Transform;

#[derive(Debug)]
pub struct Sprite {
    layer: u32,
    layer_rect: Vector4<f32>,
    pub transform: Transform,
    pub origin: Vector2<f32>,
}

impl Sprite {
    pub fn from_frame(layer: u32, layer_rect: Vector4<f32>) -> Self {
        Self {
            layer,
            layer_rect,
            transform: Transform::new(),
            origin: Vector2::new(0.5, 0.5),
        }
    }

    pub fn from_texture(texture: &Texture) -> Self {
        Self::from_frame(
            0,
            Vector4::new(0.0, 0.0, texture.width() as f32, texture.height() as f32),
        )
    }

    pub fn from_atlas(name: &str, atlas: &TextureAtlas) -> Self {
        let (layer, frame) = atlas.frame(name);
        Self::from_frame(*layer, frame.clone())
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
}

impl AsInstance for &Sprite {
    fn as_instance(&self) -> Instance {
        let (translation, rotation, scaling) = self.transform.decompose();

        Instance {
            layer: Vector3::new(self.layer as f32, self.origin[0], self.origin[1]),
            layer_rect: self.layer_rect.clone(),
            translation,
            scaling,
            rotation,
        }
    }
}
