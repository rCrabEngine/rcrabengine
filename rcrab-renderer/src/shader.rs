// Shader and PBR material support

use crate::{Error, Result};
use parking_lot::RwLock;
use rcrab_core::math::{Vec2, Vec3, Vec4};
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::{Device, ShaderModule, ShaderModuleDescriptor, ShaderSource};

/// Shader program
pub struct Shader {
    name: String,
    module: RwLock<Option<ShaderModule>>,
    params: RwLock<HashMap<String, ShaderParam>>,
}

impl Shader {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            module: RwLock::new(None),
            params: RwLock::new(HashMap::new()),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Compile from WGSL source
    pub fn compile_wgsl(&self, device: &Device, source: &str) -> Result<()> {
        let descriptor = ShaderModuleDescriptor {
            label: Some(&self.name),
            flags: wgpu::ShaderFlags::default(),
            source: ShaderSource::Wgsl(source.into()),
        };

        let module = device.create_shader_module(&descriptor);
        *self.module.write() = Some(module);

        Ok(())
    }

    /// Get the shader module
    pub fn get_module(&self) -> Option<ShaderModule> {
        self.module.read().clone()
    }

    /// Add a parameter
    pub fn add_param(&self, name: &str, param: ShaderParam) {
        self.params.write().insert(name.to_string(), param);
    }

    /// Get a parameter
    pub fn get_param(&self, name: &str) -> Option<ShaderParam> {
        self.params.read().get(name).cloned()
    }
}

/// Shader parameter types
#[derive(Debug, Clone)]
pub enum ShaderParam {
    Float(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
    Mat4(rcrab_core::math::Mat4),
    Texture(Arc<Texture>),
    Sampler(Arc<Sampler>),
}

impl ShaderParam {
    pub fn as_float(&self) -> Option<f32> {
        if let ShaderParam::Float(f) = self { Some(*f) } else { None }
    }

    pub fn as_vec3(&self) -> Option<Vec3> {
        if let ShaderParam::Vec3(v) = self { Some(*v) } else { None }
    }
}

/// Shader for PBR rendering
pub struct PbrShader {
    shader: Shader,
    vertex_shader: String,
    fragment_shader: String,
}

impl PbrShader {
    pub fn new() -> Self {
        Self {
            shader: Shader::new("PBR"),
            vertex_shader: String::new(),
            fragment_shader: String::new(),
        }
    }

    /// Get the base shader
    pub fn get_shader(&self) -> &Shader {
        &self.shader
    }

    /// Compile with standard PBR shaders
    pub fn compile_pbr(&mut self, device: &Device) -> Result<()> {
        self.vertex_shader = PBR_VERTEX_SHADER.to_string();
        self.fragment_shader = PBR_FRAGMENT_SHADER.to_string();

        // We'll compile the combined source
        let combined = format!("{}\n{}", self.vertex_shader, self.fragment_shader);
        self.shader.compile_wgsl(device, &combined)
    }
}

impl Default for PbrShader {
    fn default() -> Self {
        Self::new()
    }
}

/// Standard PBR vertex shader (WGSL)
const PBR_VERTEX_SHADER: &str = r#"
struct Uniforms {
    model_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) tangent: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) tangent: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let world_position = (uniforms.model_matrix * vec4<f32>(input.position, 1.0)).xyz;
    output.clip_position = uniforms.projection_matrix * uniforms.view_matrix * vec4<f32>(world_position, 1.0);
    output.world_position = world_position;
    output.world_normal = (uniforms.normal_matrix * vec4<f32>(input.normal, 0.0)).xyz;
    output.tex_coord = input.tex_coord;
    output.tangent = input.tangent;

    return output;
}
"#;

/// Standard PBR fragment shader (WGSL)
const PBR_FRAGMENT_SHADER: &str = r#"
struct Uniforms {
    model_matrix: mat4x4<f32>,
    view_matrix: mat4x4<f32>,
    projection_matrix: mat4x4<f32>,
    normal_matrix: mat4x4<f32>,
};

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
    intensity: f32,
    _pad: f32,
};

struct PbrParams {
    base_color: vec4<f32>,
    emissive: vec3<f32>,
    roughness: f32,
    metallic: f32,
    reflectance: f32,
    normal_strength: f32,
    ambient_occlusion: f32,
    opacity: f32,
    _pad: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) tex_coord: vec2<f32>,
    @location(3) tangent: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<uniform> params: PbrParams;
@group(0) @binding(2) var base_color_texture: texture_2d<f32>;
@group(0) @binding(3) var base_color_sampler: sampler;
@group(0) @binding(4) var normal_texture: texture_2d<f32>;
@group(0) @binding(5) var normal_sampler: sampler;
@group(0) @binding(6) var metallic_roughness_texture: texture_2d<f32>;
@group(0) @binding(7) var metallic_roughness_sampler: sampler;
@group(0) @binding(8) var occlusion_texture: texture_2d<f32>;
@group(0) @binding(9) var occlusion_sampler: sampler;
@group(0) @binding(10) var emissive_texture: texture_2d<f32>;
@group(0) @binding(11) var emissive_sampler: sampler;

// Number of lights
const NUM_LIGHTS: usize = 16;
@group(0) @binding(12) var<uniform> lights: array<Light, NUM_LIGHTS>;
@group(0) @binding(13) var num_lights: f32;

const PI: f32 = 3.14159265359;

// PBR functions
fn distribution_ggx(NdotH: f32, roughness: f32) -> f32 {
    let a = roughness * roughness;
    let a2 = a * a;
    let NdotH2 = NdotH * NdotH;
    let num = a2;
    let denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;
    return num / denom;
}

fn geometry_schlick_ggx(NdotV: f32, roughness: f32) -> f32 {
    let r = roughness + 1.0;
    let k = (r * r) / 8.0;
    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;
    return num / denom;
}

fn geometry_smith(NdotV: f32, NdotL: f32, roughness: f32) -> f32 {
    let ggx2 = geometry_schlick_ggx(NdotV, roughness);
    let ggx1 = geometry_schlick_ggx(NdotL, roughness);
    return ggx1 * ggx2;
}

fn fresnel_schlick(cos_theta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - cos_theta, 0.0, 1.0), 5.0);
}

fn get_normal(tex_coord: vec2<f32>, TBN: mat3x3<f32>) -> vec3<f32> {
    let normal = textureSample(normal_texture, normal_sampler, tex_coord).rgb;
    let normal_map = normal * 2.0 - 1.0;
    return normalize(TBN * normal_map);
}

@fragment
fn main(input: VertexOutput) -> @location(0) vec4<f32> {
    // Base color
    var base_color = params.base_color;
    base_color = base_color * textureSample(base_color_texture, base_color_sampler, input.tex_coord);

    // Normal mapping
    var normal = normalize(input.world_normal);
    let T = normalize(input.tangent.xyz);
    let B = cross(normal, T) * input.tangent.w;
    let TBN = mat3x3<T, B, normal>;
    if (params.normal_strength > 0.0) {
        normal = normalize(mix(normal, get_normal(input.tex_coord, TBN), params.normal_strength));
    }

    // Metallic-roughness
    var metallic = params.metallic;
    var roughness = params.roughness;
    let mr = textureSample(metallic_roughness_texture, metallic_roughness_sampler, input.tex_coord);
    metallic = metallic * mr.b;
    roughness = roughness * mr.g;

    // Occlusion
    var ao = params.ambient_occlusion;
    ao = ao * textureSample(occlusion_texture, occlusion_sampler, input.tex_coord).r;

    // Emissive
    var emissive = params.emissive;
    emissive = emissive + textureSample(emissive_texture, emissive_sampler, input.tex_coord).rgb;

    // Calculate F0 (reflectance at normal incidence)
    let F0 = vec3<f32>(0.16 * params.reflectance * params.reflectance);
    let F0_mix = mix(F0, base_color.rgb, metallic);

    var Lo = vec3<f32>(0.0);

    // Calculate lighting
    let view_dir = normalize(-input.world_position);

    for (var i = 0u; i < u32(num_lights); i = i + 1u) {
        let light = lights[i];
        let light_dir = normalize(light.position - input.world_position);
        let half_dir = normalize(light_dir + view_dir);

        let distance = length(light.position - input.world_position);
        let attenuation = 1.0 / (distance * distance);
        let radiance = light.color * light.intensity * attenuation;

        let NdotL = max(dot(normal, light_dir), 0.0);
        let NdotV = max(dot(normal, view_dir), 0.0);
        let NdotH = max(dot(normal, half_dir), 0.0);
        let HdotV = max(dot(half_dir, view_dir), 0.0);

        // Cook-Torrance BRDF
        let NDF = distribution_ggx(NdotH, roughness);
        let G = geometry_smith(NdotV, NdotL, roughness);
        let F = fresnel_schlick(HdotV, F0_mix);

        let numerator = NDF * G * F;
        let denominator = 4.0 * NdotV * NdotL + 0.0001;
        let specular = numerator / denominator;

        let kS = F;
        let kD = (vec3<f32>(1.0) - kS) * (1.0 - metallic);

        Lo = Lo + (kD * base_color.rgb / PI + specular) * radiance * NdotL;
    }

    // Ambient
    let ambient = vec3<f32>(0.03) * base_color.rgb * ao;
    var color = ambient + Lo + emissive;

    // HDR tonemapping
    color = color / (color + vec3<f32>(1.0));

    // Gamma correction
    color = pow(color, vec3<f32>(1.0/2.2));

    return vec4<f32>(color, base_color.a * params.opacity);
}
"#;

/// Shader material wrapper
pub struct ShaderMaterial {
    shader: Shader,
    params: RwLock<PbrMaterialParams>,
}

#[derive(Debug, Clone, Default)]
pub struct PbrMaterialParams {
    pub base_color: Vec4,
    pub emissive: Vec3,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
    pub normal_strength: f32,
    pub ambient_occlusion: f32,
    pub opacity: f32,
}

impl ShaderMaterial {
    pub fn new(name: &str) -> Self {
        Self {
            shader: Shader::new(name),
            params: RwLock::new(PbrMaterialParams::default()),
        }
    }

    pub fn get_shader(&self) -> &Shader {
        &self.shader
    }

    pub fn get_params(&self) -> PbrMaterialParams {
        self.params.read().clone()
    }

    pub fn set_base_color(&self, color: Vec4) {
        self.params.write().base_color = color;
    }

    pub fn set_roughness(&self, roughness: f32) {
        self.params.write().roughness = roughness;
    }

    pub fn set_metallic(&self, metallic: f32) {
        self.params.write().metallic = metallic;
    }
}
