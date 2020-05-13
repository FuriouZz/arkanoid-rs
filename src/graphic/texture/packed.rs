use super::RawTexture2D;
use super::Texture2DAtlas;
use crate::graphic::Gpu;
use nalgebra::Vector4;
use image::DynamicImage;

pub struct TexturePacked {
    bytes: Vec<u8>,
    width: u32,
    height: u32,
    rectangles: Vec<Vector4<u32>>,
}

impl TexturePacked {
    pub fn from_image(img: &DynamicImage) -> Self {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        Self::from_bytes(width, height, bytes)
    }

    pub fn from_bytes(width: u32, height: u32, bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            width,
            height,
            rectangles: Vec::new(),
        }
    }

    pub fn add_rect(mut self, rect: Vector4<u32>) -> Self {
        self.rectangles.push(rect);
        self
    }

    pub fn to_atlas(self, gpu: &mut Gpu, atlas: &mut Texture2DAtlas) {
        let Self {
            width,
            height,
            bytes,
            rectangles,
        } = self;

        let texture = RawTexture2D::from_bytes(
            gpu,
            width,
            height,
            wgpu::TextureUsage::COPY_SRC,
            bytes.as_slice(),
        );

        for rect in rectangles {
            atlas.append_raw_texture(gpu, &texture, rect);
        }
    }

    pub fn into_atlas(self, gpu: &mut Gpu) -> Texture2DAtlas {
        let mut atlas = Texture2DAtlas::from_layout_count(
            gpu,
            self.width,
            self.height,
            self.rectangles.len() as u32,
        );

        self.to_atlas(gpu, &mut atlas);

        atlas
    }
}
