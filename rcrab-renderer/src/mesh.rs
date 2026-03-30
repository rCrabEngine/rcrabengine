// GPU mesh handling

use crate::{Error, Result};
use parking_lot::RwLock;
use rcrab_core::scene::Mesh;
use std::sync::Arc;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Device};

/// GPU vertex buffer
pub struct GpuMesh {
    name: String,
    vertex_buffer: RwLock<Option<Buffer>>,
    index_buffer: RwLock<Option<Buffer>>,
    vertex_count: usize,
    index_count: usize,
    primitive_type: wgpu::PrimitiveTopology,
}

impl GpuMesh {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            vertex_buffer: RwLock::new(None),
            index_buffer: RwLock::new(None),
            vertex_count: 0,
            index_count: 0,
            primitive_type: wgpu::PrimitiveTopology::TriangleList,
        }
    }

    /// Create from core mesh
    pub fn from_mesh(device: &Device, mesh: &Mesh) -> Result<Self> {
        let mut gpu_mesh = Self::new(&mesh.get_name());

        // Get positions
        let positions = mesh.get_positions();
        let normals = mesh.get_normals();
        let colors = mesh.get_colors();
        let tex_coords = mesh.get_all_tex_coords();

        let vertex_count = positions.len();

        // Build vertex data
        let mut vertices: Vec<MeshVertex> = Vec::with_capacity(vertex_count);

        for i in 0..vertex_count {
            let pos = positions.get(i).copied().unwrap_or_default();
            let normal = normals.get(i).copied().unwrap_or_default();
            let color = colors.as_ref()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(glam::Vec4::new(1.0, 1.0, 1.0, 1.0));
            let uv0 = tex_coords.get(0)
                .and_then(|uv| uv.get(i))
                .copied()
                .unwrap_or_default();
            let tangent = mesh.get_tangents()
                .and_then(|t| t.get(i))
                .copied()
                .unwrap_or(glam::Vec4::new(1.0, 0.0, 0.0, 1.0));

            vertices.push(MeshVertex {
                position: pos,
                normal,
                tex_coord: uv0,
                tangent,
                color,
            });
        }

        // Create vertex buffer
        let vertex_data = bytemuck::cast_slice(&vertices);
        let vertex_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: vertex_data.len() as u64,
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        gpu_mesh.vertex_buffer = RwLock::new(Some(vertex_buffer));
        gpu_mesh.vertex_count = vertex_count;

        // Create index buffer if we have indices
        if let Some(indices) = mesh.get_indices() {
            let index_data = bytemuck::cast_slice(&indices);
            let index_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("Index Buffer"),
                size: index_data.len() as u64,
                usage: BufferUsages::INDEX | BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            gpu_mesh.index_buffer = RwLock::new(Some(index_buffer));
            gpu_mesh.index_count = indices.len();
        }

        // Set primitive type
        gpu_mesh.primitive_type = match mesh.get_primitive_type() {
            rcrab_core::scene::PrimitiveType::Triangles => wgpu::PrimitiveTopology::TriangleList,
            rcrab_core::scene::PrimitiveType::Lines => wgpu::PrimitiveTopology::LineList,
            rcrab_core::scene::PrimitiveType::Points => wgpu::PrimitiveTopology::PointList,
            rcrab_core::scene::PrimitiveType::TriangleStrip => wgpu::PrimitiveTopology::TriangleStrip,
            rcrab_core::scene::PrimitiveType::LineStrip => wgpu::PrimitiveTopology::LineStrip,
        };

        Ok(gpu_mesh)
    }

    /// Get vertex buffer
    pub fn get_vertex_buffer(&self) -> Option<Buffer> {
        self.vertex_buffer.read().clone()
    }

    /// Get index buffer
    pub fn get_index_buffer(&self) -> Option<Buffer> {
        self.index_buffer.read().clone()
    }

    /// Get vertex count
    pub fn get_vertex_count(&self) -> usize {
        self.vertex_count
    }

    /// Get index count
    pub fn get_index_count(&self) -> usize {
        self.index_count
    }

    /// Has index buffer
    pub fn has_indices(&self) -> bool {
        self.index_count > 0
    }

    /// Get primitive type
    pub fn get_primitive_type(&self) -> wgpu::PrimitiveTopology {
        self.primitive_type
    }

    /// Upload vertex data
    pub fn upload_vertices(&self, queue: &wgpu::Queue, data: &[u8]) {
        if let Some(buffer) = self.vertex_buffer.read().as_ref() {
            queue.write_buffer(buffer, 0, data);
        }
    }

    /// Upload index data
    pub fn upload_indices(&self, queue: &wgpu::Queue, data: &[u8]) {
        if let Some(buffer) = self.index_buffer.read().as_ref() {
            queue.write_buffer(buffer, 0, data);
        }
    }
}

/// Vertex format for rendering
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct MeshVertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub tex_coord: glam::Vec2,
    pub tangent: glam::Vec4,
    pub color: glam::Vec4,
}

unsafe impl Pod {}
unsafe impl Zeroable {}

/// Mesh vertex attributes for wgpu
pub const MESH_VERTEX_ATTRIBUTES: &[wgpu::VertexAttribute] = &[
    wgpu::VertexAttribute {
        shader_location: 0,
        offset: 0,
        format: wgpu::VertexFormat::Float32x3, // position
    },
    wgpu::VertexAttribute {
        shader_location: 1,
        offset: 12,
        format: wgpu::VertexFormat::Float32x3, // normal
    },
    wgpu::VertexAttribute {
        shader_location: 2,
        offset: 24,
        format: wgpu::VertexFormat::Float32x2, // tex_coord
    },
    wgpu::VertexAttribute {
        shader_location: 3,
        offset: 32,
        format: wgpu::VertexFormat::Float32x4, // tangent
    },
    wgpu::VertexAttribute {
        shader_location: 4,
        offset: 48,
        format: wgpu::VertexFormat::Float32x4, // color
    },
];

/// Vertex buffer layout
pub const MESH_VERTEX_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<MeshVertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: MESH_VERTEX_ATTRIBUTES,
};

/// Add bytemuck for glam types
mod bytemuck {
    use glam::{Vec2, Vec3, Vec4};

    unsafe impl Pod for Vec2 {}
    unsafe impl Pod for Vec3 {}
    unsafe impl Pod for Vec4 {}

    unsafe impl Zeroable for Vec2 {
        fn zeroed() -> Self {
            Self::ZERO
        }
    }

    unsafe impl Zeroable for Vec3 {
        fn zeroed() -> Self {
            Self::ZERO
        }
    }

    unsafe impl Zeroable for Vec4 {
        fn zeroed() -> Self {
            Self::ZERO
        }
    }
}
