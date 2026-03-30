// Render pipeline

use crate::{mesh::MESH_VERTEX_LAYOUT, Error, Result};
use parking_lot::RwLock;
use rcrab_core::scene::Camera;
use std::sync::Arc;
use wgpu::{Device, PipelineLayout, RenderPipeline, RenderPipelineDescriptor, ShaderModule, VertexState, FragmentState, ColorTargetState, DepthStencilState, PolygonMode, FrontFace, CullMode, PrimitiveState, BlendState, BlendComponent};

/// Pipeline state
pub struct PipelineState {
    pub layout: RwLock<Option<PipelineLayout>>,
    pub pipeline: RwLock<Option<RenderPipeline>>,
}

impl PipelineState {
    pub fn new() -> Self {
        Self {
            layout: RwLock::new(None),
            pipeline: RwLock::new(None),
        }
    }

    pub fn get_pipeline(&self) -> Option<RenderPipeline> {
        self.pipeline.read().clone()
    }
}

impl Default for PipelineState {
    fn default() -> Self {
        Self::new()
    }
}

/// Render pipeline builder
pub struct RenderPipelineBuilder {
    name: String,
    vertex_shader: Option<ShaderModule>,
    fragment_shader: Option<ShaderModule>,
    vertex_layouts: Vec<wgpu::VertexBufferLayout>,
    depth_stencil: Option<wgpu::DepthStencilState>,
    blend: Option<wgpu::BlendState>,
    cull_mode: wgpu::CullMode,
    polygon_mode: wgpu::PolygonMode,
}

impl RenderPipelineBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            vertex_shader: None,
            fragment_shader: None,
            vertex_layouts: vec![MESH_VERTEX_LAYOUT],
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            blend: Some(wgpu::BlendState::ALPHA_BLEND),
            cull_mode: wgpu::CullMode::Back,
            polygon_mode: wgpu::PolygonMode::Fill,
        }
    }

    pub fn with_vertex_shader(mut self, shader: ShaderModule) -> Self {
        self.vertex_shader = Some(shader);
        self
    }

    pub fn with_fragment_shader(mut self, shader: ShaderModule) -> Self {
        self.fragment_shader = Some(shader);
        self
    }

    pub fn with_depth_stencil(mut self, enabled: bool) -> Self {
        if enabled {
            self.depth_stencil = Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            });
        } else {
            self.depth_stencil = None;
        }
        self
    }

    pub fn with_blend(mut self, enabled: bool) -> Self {
        if enabled {
            self.blend = Some(wgpu::BlendState::ALPHA_BLEND);
        } else {
            self.blend = None;
        }
        self
    }

    pub fn with_cull_mode(mut self, mode: wgpu::CullMode) -> Self {
        self.cull_mode = mode;
        self
    }

    pub fn with_polygon_mode(mut self, mode: wgpu::PolygonMode) -> Self {
        self.polygon_mode = mode;
        self
    }

    pub fn build(self, device: &Device, layout: &PipelineLayout) -> Result<RenderPipeline> {
        let vertex = self.vertex_shader.ok_or_else(|| Error::Pipeline("No vertex shader".into()))?;
        let fragment = self.fragment_shader.ok_or_else(|| Error::Pipeline("No fragment shader".into()))?;

        let descriptor = RenderPipelineDescriptor {
            label: Some(&self.name),
            layout: Some(layout),
            vertex: VertexState {
                module: &vertex,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                buffers: &self.vertex_layouts,
            },
            fragment: Some(FragmentState {
                module: &fragment,
                entry_point: Some("main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    blend: self.blend,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(self.cull_mode),
                polygon_fill_mode: self.polygon_mode,
                conservative: false,
            },
            depth_stencil: self.depth_stencil,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: Default::default(),
            cache: None,
        };

        Ok(device.create_render_pipeline(&descriptor))
    }
}

/// Pipeline layout builder
pub struct PipelineLayoutBuilder {
    name: String,
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
}

impl PipelineLayoutBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            bind_group_layouts: Vec::new(),
        }
    }

    pub fn with_bind_group_layout(mut self, layout: wgpu::BindGroupLayout) -> Self {
        self.bind_group_layouts.push(layout);
        self
    }

    pub fn build(self, device: &Device) -> wgpu::PipelineLayout {
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some(&self.name),
            bind_group_layouts: &self.bind_group_layouts,
            immediate_size: 0,
        })
    }
}

/// Uniform buffer for matrices
#[derive(Debug, Clone, Copy)]
pub struct MatrixUniforms {
    pub model: rcrab_core::math::Mat4,
    pub view: rcrab_core::math::Mat4,
    pub projection: rcrab_core::math::Mat4,
    pub normal_matrix: rcrab_core::math::Mat4,
}

impl Default for MatrixUniforms {
    fn default() -> Self {
        Self {
            model: rcrab_core::math::Mat4::IDENTITY,
            view: rcrab_core::math::Mat4::IDENTITY,
            projection: rcrab_core::math::Mat4::IDENTITY,
            normal_matrix: rcrab_core::math::Mat4::IDENTITY,
        }
    }
}

/// Create a default PBR pipeline layout
pub fn create_pbr_pipeline_layout(device: &Device) -> wgpu::PipelineLayout {
    let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("PBR Uniforms"),
        entries: &[
            // Matrices
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // PBR Params
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // Base color texture
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Base color sampler
            wgpu::BindGroupLayoutEntry {
                binding: 3,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
            // Normal texture
            wgpu::BindGroupLayoutEntry {
                binding: 4,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Normal sampler
            wgpu::BindGroupLayoutEntry {
                binding: 5,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
            // Metallic-roughness texture
            wgpu::BindGroupLayoutEntry {
                binding: 6,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Metallic-roughness sampler
            wgpu::BindGroupLayoutEntry {
                binding: 7,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
            // Occlusion texture
            wgpu::BindGroupLayoutEntry {
                binding: 8,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Occlusion sampler
            wgpu::BindGroupLayoutEntry {
                binding: 9,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
            // Emissive texture
            wgpu::BindGroupLayoutEntry {
                binding: 10,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // Emissive sampler
            wgpu::BindGroupLayoutEntry {
                binding: 11,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler { comparison: false },
                count: None,
            },
            // Lights
            wgpu::BindGroupLayoutEntry {
                binding: 12,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            // Num lights
            wgpu::BindGroupLayoutEntry {
                binding: 13,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    PipelineLayoutBuilder::new("PBR Layout")
        .with_bind_group_layout(uniform_layout)
        .build(device)
}
