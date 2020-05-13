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
    // pub fn from_texture(texture: &Texture) -> Self {
    //     let (width, height) = texture.get_dimension();
    //     Self {
    //         layer: 0,
    //         translation: Vector3::new(0.0, 0.0, 0.0),
    //         scaling: Vector3::new(1.0, 1.0, 1.0),
    //         rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
    //         origin: Vector2::new(
    //             width as f32 * -0.5,
    //             height as f32 * -0.5,
    //         ),
    //         layer_rect: Vector4::new(0.0, 0.0, width as f32, height as f32),
    //     }
    // }

    pub fn from_atlas(atlas: &TextureAtlas, name: &str) -> Self {
        let (layer, rectangle) = atlas.rectangle(name);
        Self {
            layer: *layer,
            translation: Vector3::new(0.0, 0.0, 0.0),
            scaling: Vector3::new(1.0, 1.0, 1.0),
            rotation: UnitQuaternion::from_euler_angles(0.0, 0.0, 0.0),
            origin: Vector2::new(
                rectangle[2] as f32 * -0.5,
                rectangle[3] as f32 * -0.5,
            ),
            layer_rect: rectangle.clone(),
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
        self.scaling[0] * self.layer_rect[2]
    }

    pub fn height(&self) -> f32 {
        self.scaling[1] * self.layer_rect[3]
    }

    pub(super) fn as_instance(&self) -> SpriteInstance {
        SpriteInstance {
            layer: self.layer,
            translation: Vector3::new(
                self.translation[0] + self.origin[0],
                self.translation[1] + self.origin[1],
                self.translation[2],
            ),
            rotation: self.rotation.clone(),
            scaling: Vector3::new(
                self.width(),
                self.height(),
                self.scaling[2]
            ),
            layer_rect: self.layer_rect.clone()
        }
    }
}
