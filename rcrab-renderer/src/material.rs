// Material system

use crate::{shader::PbrMaterialParams, texture::Texture2D, Error, Result};
use parking_lot::RwLock;
use rcrab_core::math::Vec3;
use std::sync::Arc;
use wgpu::Device;

/// PBR Material
pub struct PbrMaterial {
    name: String,
    params: RwLock<PbrMaterialParams>,
    textures: RwLock<PbrTextures>,
    pub bound: RwLock<bool>,
}

#[derive(Debug, Clone, Default)]
pub struct PbrTextures {
    pub base_color: Option<Arc<Texture2D>>,
    pub normal: Option<Arc<Texture2D>>,
    pub metallic_roughness: Option<Arc<Texture2D>>,
    pub occlusion: Option<Arc<Texture2D>>,
    pub emissive: Option<Arc<Texture2D>>,
}

impl PbrMaterial {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            params: RwLock::new(PbrMaterialParams::default()),
            textures: RwLock::new(PbrTextures::default()),
            bound: RwLock::new(false),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Set base color factor
    pub fn set_base_color(&self, color: rcrab_core::math::Vec4) {
        self.params.write().base_color = color;
    }

    /// Set base color from RGB
    pub fn set_color(&self, r: f32, g: f32, b: f32) {
        self.params.write().base_color = rcrab_core::math::Vec4::new(r, g, b, 1.0);
    }

    /// Set metallic factor
    pub fn set_metallic(&self, metallic: f32) {
        self.params.write().metallic = metallic;
    }

    /// Set roughness factor
    pub fn set_roughness(&self, roughness: f32) {
        self.params.write().roughness = roughness;
    }

    /// Set reflectance
    pub fn set_reflectance(&self, reflectance: f32) {
        self.params.write().reflectance = reflectance;
    }

    /// Set emissive color
    pub fn set_emissive(&self, emissive: Vec3) {
        self.params.write().emissive = emissive;
    }

    /// Set normal strength
    pub fn set_normal_strength(&self, strength: f32) {
        self.params.write().normal_strength = strength;
    }

    /// Set ambient occlusion factor
    pub fn set_ambient_occlusion(&self, ao: f32) {
        self.params.write().ambient_occlusion = ao;
    }

    /// Set opacity
    pub fn set_opacity(&self, opacity: f32) {
        self.params.write().opacity = opacity;
    }

    /// Set base color texture
    pub fn set_base_color_texture(&self, texture: Arc<Texture2D>) {
        self.textures.write().base_color = Some(texture);
    }

    /// Set normal texture
    pub fn set_normal_texture(&self, texture: Arc<Texture2D>) {
        self.textures.write().normal = Some(texture);
    }

    /// Set metallic-roughness texture
    pub fn set_metallic_roughness_texture(&self, texture: Arc<Texture2D>) {
        self.textures.write().metallic_roughness = Some(texture);
    }

    /// Set occlusion texture
    pub fn set_occlusion_texture(&self, texture: Arc<Texture2D>) {
        self.textures.write().occlusion = Some(texture);
    }

    /// Set emissive texture
    pub fn set_emissive_texture(&self, texture: Arc<Texture2D>) {
        self.textures.write().emissive = Some(texture);
    }

    /// Get parameters
    pub fn get_params(&self) -> PbrMaterialParams {
        self.params.read().clone()
    }

    /// Get textures
    pub fn get_textures(&self) -> PbrTextures {
        self.textures.read().clone()
    }

    /// Check if has base color texture
    pub fn has_base_color_texture(&self) -> bool {
        self.textures.read().base_color.is_some()
    }

    /// Check if has normal texture
    pub fn has_normal_texture(&self) -> bool {
        self.textures.read().normal.is_some()
    }
}

impl Default for PbrMaterial {
    fn default() -> Self {
        let mat = Self::new("Default");
        mat.set_base_color(rcrab_core::math::Vec4::new(0.8, 0.8, 0.8, 1.0));
        mat.set_metallic(0.0);
        mat.set_roughness(0.5);
        mat.set_reflectance(0.5);
        mat.set_normal_strength(1.0);
        mat.set_ambient_occlusion(1.0);
        mat.set_opacity(1.0);
        mat
    }
}

/// Material instance for rendering
pub struct MaterialInstance {
    material: Arc<PbrMaterial>,
    params_buffer: RwLock<Option<wgpu::Buffer>>,
}

impl MaterialInstance {
    pub fn new(material: Arc<PbrMaterial>) -> Self {
        Self {
            material,
            params_buffer: RwLock::new(None),
        }
    }

    /// Create and upload params buffer
    pub fn create_params_buffer(&self, device: &Device) -> Result<wgpu::Buffer> {
        let params = self.material.get_params();
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("PBR Params Buffer"),
            size: std::mem::size_of::<PbrMaterialParams>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        *self.params_buffer.write() = Some(buffer.clone());
        Ok(buffer)
    }

    /// Update params buffer
    pub fn update_params(&self, queue: &wgpu::Queue) {
        if let Some(buffer) = self.params_buffer.read().as_ref() {
            let params = self.material.get_params();
            let data = bytemuck::bytes_of(&params);
            queue.write_buffer(buffer, 0, data);
        }
    }

    /// Get the material
    pub fn get_material(&self) -> &Arc<PbrMaterial> {
        &self.material
    }

    /// Get params buffer
    pub fn get_params_buffer(&self) -> Option<wgpu::Buffer> {
        self.params_buffer.read().clone()
    }
}

/// Create a simple colored material
pub fn create_color_material(r: f32, g: f32, b: f32) -> Arc<PbrMaterial> {
    let mat = Arc::new(PbrMaterial::new("Color"));
    mat.set_color(r, g, b);
    mat
}

/// Create a simple metallic material
pub fn create_metallic_material(color: rcrab_core::math::Vec3, metallic: f32, roughness: f32) -> Arc<PbrMaterial> {
    let mat = Arc::new(PbrMaterial::new("Metallic"));
    mat.set_base_color(rcrab_core::math::Vec4::new(color.x, color.y, color.z, 1.0));
    mat.set_metallic(metallic);
    mat.set_roughness(roughness);
    mat
}
