// Texture support

use crate::{Error, Result};
use parking_lot::RwLock;
use std::sync::Arc;
use wgpu::{Device, Extent3d, Texture as WgpuTexture, TextureDescriptor, TextureDimension, TextureFormat, TextureUsage};

/// Base texture trait
pub trait Texture: Send + Sync {
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_format(&self) -> TextureFormat;
}

/// 2D texture
pub struct Texture2D {
    width: u32,
    height: u32,
    format: TextureFormat,
    texture: RwLock<Option<WgpuTexture>>,
    view: RwLock<Option<wgpu::TextureView>>,
}

impl Texture2D {
    pub fn new(width: u32, height: u32, format: TextureFormat) -> Self {
        Self {
            width,
            height,
            format,
            texture: RwLock::new(None),
            view: RwLock::new(None),
        }
    }

    /// Create from image data
    pub fn from_image(device: &Device, data: &[u8], width: u32, height: u32) -> Result<Self> {
        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture2D"),
            size: Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsage::TEXTURE_BINDING | TextureUsage::COPY_DST,
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut tex = Self {
            width,
            height,
            format: TextureFormat::Rgba8Unorm,
            texture: RwLock::new(Some(texture)),
            view: RwLock::new(Some(view)),
        };

        Ok(tex)
    }

    /// Get the texture view
    pub fn get_view(&self) -> Option<wgpu::TextureView> {
        self.view.read().clone()
    }

    /// Get the underlying texture
    pub fn get_texture(&self) -> Option<WgpuTexture> {
        self.texture.read().clone()
    }

    /// Upload data to the texture
    pub fn upload(&self, device: &Device, queue: &wgpu::Queue, data: &[u8]) {
        if let Some(texture) = self.texture.read().as_ref() {
            queue.write_texture(
                wgpu::ImageCopyTexture {
                    texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                },
                data,
                wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(self.width * 4),
                    rows_per_image: Some(self.height),
                },
                Extent3d {
                    width: self.width,
                    height: self.height,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
}

impl Texture for Texture2D {
    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn get_format(&self) -> TextureFormat {
        self.format
    }
}

/// Sampler for textures
pub struct Sampler {
    sampler: RwLock<Option<wgpu::Sampler>>,
}

impl Sampler {
    pub fn new(device: &Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Default Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            sampler: RwLock::new(Some(sampler)),
        }
    }

    pub fn get_sampler(&self) -> Option<wgpu::Sampler> {
        self.sampler.read().clone()
    }

    pub fn create_bilinear(device: &Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Bilinear Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            sampler: RwLock::new(Some(sampler)),
        }
    }

    pub fn create_trilinear(device: &Device) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Trilinear Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            sampler: RwLock::new(Some(sampler)),
        }
    }

    pub fn create_anisotropic(device: &Device, max_anisotropy: u16) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Anisotropic Sampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            max_anisotropy,
            ..Default::default()
        });

        Self {
            sampler: RwLock::new(Some(sampler)),
        }
    }
}
