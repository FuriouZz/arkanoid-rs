use super::{AsInstance, Instance};
use fine::graphic::TextureAtlas;
use fine::math::{UnitQuaternion, Vector2, Vector3, Vector4};
use fine::Transform;
use std::rc::Rc;

pub struct MovieClip {
    layer: u32,
    layer_rect: Vector4<f32>,
    texture: Rc<TextureAtlas>,
    pub transform: Transform,
    pub origin: Vector2<f32>,
}

impl MovieClip {
    pub fn new(frame: impl Into<String>, texture: Rc<TextureAtlas>) -> Self {
        let (layer, layer_rect) = texture.frame(frame).expect("No frame found");
        Self {
            layer,
            layer_rect,
            texture,
            transform: Transform::new(),
            origin: Vector2::new(0.5, 0.5),
        }
    }

    pub fn set_atlas(&mut self, frame: impl Into<String>, texture: Rc<TextureAtlas>) {
        self.texture = texture;
        self.set_frame(frame);
    }

    pub fn set_frame(&mut self, frame: impl Into<String>) {
        let (layer, layer_rect) = self.texture.frame(frame).expect("No frame found");
        self.layer = layer;
        self.layer_rect = layer_rect;
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

impl AsInstance for &MovieClip {
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
