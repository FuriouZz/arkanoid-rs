use crate::graphic::Gpu;
use image::DynamicImage;
use nalgebra::Vector4;
use std::collections::HashMap;
use super::RawTexture2D;

pub struct Texture2DAtlas {
    layer_count: u32,
    width: u32,
    height: u32,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    layers: HashMap<u32, (u32, u32)>,
}

impl Texture2DAtlas {
    const MAX_LAYER_COUNT: usize = 256;

    pub fn new(gpu: &mut Gpu, width: u32, height: u32) -> Self {
        Self::from_layout_count(gpu, width, height, Texture2DAtlas::MAX_LAYER_COUNT as u32)
    }

    pub fn from_layout_count(gpu: &mut Gpu, width: u32, height: u32, layer_count: u32) -> Self {
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("[fine::graphic::TextureAtlas] texture"),
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureDimension::D2,
            size: wgpu::Extent3d {
                width,
                height,
                depth: layer_count,
            },
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor {
            format: wgpu::TextureFormat::Bgra8Unorm,
            dimension: wgpu::TextureViewDimension::D2Array,
            aspect: wgpu::TextureAspect::default(),
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            array_layer_count: layer_count,
        });

        Self {
            layer_count,
            width,
            height,
            texture,
            view,
            layers: HashMap::new(),
        }
    }

    pub fn as_view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn get_layer_dimension(&self, layer: u32) -> &(u32, u32) {
        self.layers.get(&layer).expect("Layer does not exist.")
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn append_bytes(&mut self, gpu: &mut Gpu, width: u32, height: u32, bytes: &[u8]) {
        self.assert(width, height);

        let copy = gpu
            .device
            .create_buffer_with_data(bytes, wgpu::BufferUsage::COPY_SRC);

        let array_layer = self.layers.len() as u32;

        gpu.encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &copy,
                offset: 0,
                bytes_per_row: 4 * width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                array_layer,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width,
                height,
                depth: 1,
            },
        );

        let _ = self.layers.insert(array_layer, (width, height));
    }

    pub fn append_image(&mut self, gpu: &mut Gpu, img: DynamicImage) {
        let img = img.to_bgra();
        let (width, height) = img.dimensions();
        let bytes = &img.into_raw()[..];
        self.append_bytes(gpu, width, height, bytes);
    }

    pub fn append_raw_texture(&mut self, gpu: &mut Gpu, texture: &RawTexture2D, rectangle: Vector4<u32>) {
        self.assert(rectangle[2], rectangle[3]);

        let array_layer = self.layers.len() as u32;

        gpu.encoder.copy_texture_to_texture(
            wgpu::TextureCopyView {
                texture: texture.as_raw(),
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d {
                    x: rectangle[0],
                    y: rectangle[1],
                    z: 0,
                },
            },
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                array_layer,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::Extent3d {
                width: rectangle[2],
                height: rectangle[3],
                depth: 1,
            },
        );

        let _ = self.layers.insert(array_layer, (rectangle[2], rectangle[3]));
    }

    fn assert(&self, width: u32, height: u32) {
        assert!(
            self.layers.len() as u32 <= self.layer_count,
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
