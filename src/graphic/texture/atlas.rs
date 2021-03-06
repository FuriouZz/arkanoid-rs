use super::{RawTexture, Texture};
use crate::graphic::Gpu;
use image::DynamicImage;
use nalgebra::{Vector2, Vector4};
use std::collections::HashMap;

pub struct TextureAtlas {
    layer_count: u32,
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    rectangles: HashMap<String, (u32, Vector4<f32>)>,
}

impl TextureAtlas {
    const MAX_LAYER_COUNT: usize = 256;

    pub fn new(gpu: &mut Gpu, width: u32, height: u32, layout_count: u32) -> Self {
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("[fine::graphic::TextureAtlas] texture"),
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureDimension::D2,
            size: wgpu::Extent3d {
                width,
                height,
                depth: layout_count,
            },
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_SRC
                | wgpu::TextureUsage::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureViewDimension::D2Array,
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            array_layer_count: layout_count,
        });

        Self {
            layer_count: layout_count,
            width,
            height,
            texture,
            view,
            rectangles: HashMap::new(),
        }
    }

    pub fn frame(&self, name: impl Into<String>) -> Option<(u32, Vector4<f32>)> {
        self.rectangles.get(&name.into()).map(|r| r.clone())
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn layer_count(&self) -> u32 {
        self.layer_count
    }

    pub fn dimensions(&self) -> (u32, u32, u32) {
        (self.width, self.height, self.layer_count)
    }

    pub fn append_raw_texture<S>(
        &mut self,
        name: S,
        layer: u32,
        gpu: &mut Gpu,
        texture: &RawTexture,
        source: Vector4<u32>,
        destination: Option<Vector2<u32>>,
    ) where
        S: Into<String>,
    {
        self.assert(source[2], source[3], layer);

        let origin = if let Some(origin) = destination {
            wgpu::Origin3d {
                x: origin[0],
                y: origin[1],
                z: 0,
            }
        } else {
            wgpu::Origin3d::ZERO
        };

        gpu.encoder.copy_texture_to_texture(
            wgpu::TextureCopyView {
                texture: texture.as_raw(),
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d {
                    x: source[0],
                    y: source[1],
                    z: 0,
                },
            },
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                array_layer: layer,
                origin,
            },
            wgpu::Extent3d {
                width: source[2],
                height: source[3],
                depth: 1,
            },
        );

        let _ = self.rectangles.insert(
            name.into(),
            (
                layer,
                Vector4::new(
                    origin.x as f32,
                    origin.y as f32,
                    source[2] as f32,
                    source[3] as f32,
                ),
            ),
        );
    }

    pub fn append_bytes<F>(
        &mut self,
        gpu: &mut Gpu,
        bytes: &[u8],
        width: u32,
        height: u32,
        mut f: F,
    ) where
        F: FnMut(&mut TextureAtlasFromBytes),
    {
        let texture =
            &RawTexture::from_bytes(gpu, width, height, wgpu::TextureUsage::COPY_SRC, bytes);

        f(&mut TextureAtlasFromBytes {
            gpu,
            texture,
            atlas: self,
        });
    }

    pub fn append_image<F>(&mut self, gpu: &mut Gpu, img: &DynamicImage, f: F)
    where
        F: FnMut(&mut TextureAtlasFromBytes),
    {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        self.append_bytes(gpu, bytes, width, height, f);
    }

    pub fn frame_to_raw(&self, gpu: &mut Gpu, name: impl Into<String>) -> Option<RawTexture> {
        self.frame(name).map(|frame| {
            let width = frame.1[2] as u32;
            let height = frame.1[3] as u32;

            let raw = gpu.device.create_texture(&wgpu::TextureDescriptor {
                label: Some("[fine::graphic::TextureAtlas] frame_to_raw_texture"),
                format: wgpu::TextureFormat::Bgra8Unorm,
                dimension: wgpu::TextureDimension::D2,
                size: wgpu::Extent3d {
                    width,
                    height,
                    depth: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                usage: wgpu::TextureUsage::COPY_DST,
            });

            gpu.encoder.copy_texture_to_texture(
                wgpu::TextureCopyView {
                    texture: &self.texture,
                    array_layer: frame.0,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: frame.1[0] as u32,
                        y: frame.1[1] as u32,
                        z: 0,
                    }
                },
                wgpu::TextureCopyView {
                    texture: &raw,
                    array_layer: 0,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth: 1,
                },
            );

            RawTexture::new(raw, width, height)
        })
    }

    pub fn frame_to_texture(&self, gpu: &mut Gpu, name: impl Into<String>) -> Option<Texture> {
        self.frame_to_raw(gpu, name).map(|raw| {
            let width = raw.width();
            let height = raw.height();
            Texture::from_raw(&raw, width, height)
        })
    }

    fn assert(&self, width: u32, height: u32, layer: u32) {
        assert!(
            layer <= self.layer_count,
            "[fine::graphic::TextureAtlas] Cannot add more layers."
        );
        assert!(
            width <= self.width,
            "[fine::graphic::TextureAtlas] width is bigger than the atlas width."
        );
        assert!(
            height <= self.height,
            "[fine::graphic::TextureAtlas] height is bigger than the atlas height."
        );
    }
}

impl super::AsTextureView for TextureAtlas {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

impl super::AsTextureView for &TextureAtlas {
    fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }
}

pub struct TextureAtlasFromBytes<'a> {
    gpu: &'a mut Gpu,
    texture: &'a RawTexture,
    atlas: &'a mut TextureAtlas,
}

impl<'a> TextureAtlasFromBytes<'a> {
    pub fn add<S>(
        &mut self,
        name: S,
        layer: u32,
        source: Vector4<u32>,
        destination: Option<Vector2<u32>>,
    ) -> &Self
    where
        S: Into<String>,
    {
        self.atlas
            .append_raw_texture(name, layer, self.gpu, &self.texture, source, destination);
        self
    }
}
