// Mesh - Geometric data

use crate::math::{bounding::BoundingSphere, BoundingVolume, Vec2, Vec3, Vec4};
use parking_lot::RwLock;
use std::sync::Arc;

/// Vertex attribute types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VertexAttribute {
    Position,
    Normal,
    Tangent,
    Binormal,
    Color,
    TexCoord(u32), // Texture coordinate set index
    StaticOffset,
}

/// Mesh data structure
pub struct Mesh {
    name: RwLock<String>,
    vertex_count: RwLock<usize>,
    positions: RwLock<Vec<Vec3>>,
    normals: RwLock<Vec<Vec3>>,
    tangents: RwLock<Option<Vec<Vec3>>>,
    binormals: RwLock<Option<Vec<Vec3>>>,
    colors: RwLock<Option<Vec<Vec4>>>,
    tex_coords: RwLock<Vec<Vec<Vec2>>>, // Multiple UV sets
    indices: RwLock<Option<Vec<u32>>>,
    bounding: RwLock<Option<BoundingVolume>>,
    primitive_type: RwLock<PrimitiveType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimitiveType {
    Triangles,
    Lines,
    Points,
    TriangleStrip,
    LineStrip,
}

impl Default for PrimitiveType {
    fn default() -> Self {
        Self::Triangles
    }
}

impl Mesh {
    pub fn new(name: &str) -> Self {
        Self {
            name: RwLock::new(name.to_string()),
            vertex_count: RwLock::new(0),
            positions: RwLock::new(Vec::new()),
            normals: RwLock::new(Vec::new()),
            tangents: RwLock::new(None),
            binormals: RwLock::new(None),
            colors: RwLock::new(None),
            tex_coords: RwLock::new(Vec::new()),
            indices: RwLock::new(None),
            bounding: RwLock::new(None),
            primitive_type: RwLock::new(PrimitiveType::Triangles),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.read().clone()
    }

    pub fn set_name(&self, name: &str) {
        *self.name.write() = name.to_string();
    }

    /// Get vertex count
    pub fn get_vertex_count(&self) -> usize {
        *self.vertex_count.read()
    }

    /// Set vertex positions
    pub fn set_positions(&self, positions: Vec<Vec3>) {
        *self.vertex_count.write() = positions.len();
        *self.positions.write() = positions;
        self.update_bounding();
    }

    /// Get positions
    pub fn get_positions(&self) -> Vec<Vec3> {
        self.positions.read().clone()
    }

    /// Set normals
    pub fn set_normals(&self, normals: Vec<Vec3>) {
        *self.normals.write() = normals;
    }

    /// Get normals
    pub fn get_normals(&self) -> Vec<Vec3> {
        self.normals.read().clone()
    }

    /// Set tangents
    pub fn set_tangents(&self, tangents: Vec<Vec3>) {
        *self.tangents.write() = Some(tangents);
    }

    /// Get tangents
    pub fn get_tangents(&self) -> Option<Vec<Vec3>> {
        self.tangents.read().clone()
    }

    /// Set binormals
    pub fn set_binormals(&self, binormals: Vec<Vec3>) {
        *self.binormals.write() = Some(binormals);
    }

    /// Set colors
    pub fn set_colors(&self, colors: Vec<Vec4>) {
        *self.colors.write() = Some(colors);
    }

    /// Get colors
    pub fn get_colors(&self) -> Option<Vec<Vec4>> {
        self.colors.read().clone()
    }

    /// Set texture coordinates for a unit
    pub fn set_tex_coords(&self, unit: usize, coords: Vec<Vec2>) {
        let mut tex_coords = self.tex_coords.write();
        while tex_coords.len() <= unit {
            tex_coords.push(Vec::new());
        }
        tex_coords[unit] = coords;
    }

    /// Get texture coordinates for a unit
    pub fn get_tex_coords(&self, unit: usize) -> Option<Vec<Vec2>> {
        self.tex_coords.read().get(unit).cloned()
    }

    /// Get all texture coordinates
    pub fn get_all_tex_coords(&self) -> Vec<Vec<Vec2>> {
        self.tex_coords.read().clone()
    }

    /// Set indices
    pub fn set_indices(&self, indices: Vec<u32>) {
        *self.indices.write() = Some(indices);
    }

    /// Get indices
    pub fn get_indices(&self) -> Option<Vec<u32>> {
        self.indices.read().clone()
    }

    /// Get bounding volume
    pub fn get_bounding(&self) -> Option<BoundingVolume> {
        self.bounding.read().clone()
    }

    /// Set bounding volume
    pub fn set_bounding(&self, bounding: BoundingVolume) {
        *self.bounding.write() = Some(bounding);
    }

    /// Get primitive type
    pub fn get_primitive_type(&self) -> PrimitiveType {
        *self.primitive_type.read()
    }

    /// Set primitive type
    pub fn set_primitive_type(&self, ptype: PrimitiveType) {
        *self.primitive_type.write() = ptype;
    }

    /// Update bounding from positions
    fn update_bounding(&self) {
        let positions = self.positions.read();
        if positions.is_empty() {
            return;
        }

        // Compute center and radius for sphere
        let center = positions.iter().fold(Vec3::ZERO, |acc, p| acc + *p)
            / positions.len() as f32;

        let radius = positions
            .iter()
            .map(|p| (*p - center).length())
            .fold(0.0_f32, |a, b| a.max(b));

        *self.bounding.write() = Some(BoundingVolume::Sphere(BoundingSphere::new(center, radius)));
    }

    /// Check if mesh has a specific attribute
    pub fn has_attribute(&self, attr: VertexAttribute) -> bool {
        match attr {
            VertexAttribute::Position => !self.positions.read().is_empty(),
            VertexAttribute::Normal => !self.normals.read().is_empty(),
            VertexAttribute::Tangent => self.tangents.read().is_some(),
            VertexAttribute::Binormal => self.binormals.read().is_some(),
            VertexAttribute::Color => self.colors.read().is_some(),
            VertexAttribute::TexCoord(i) => self.tex_coords.read().get(i as usize).map(|c: &Vec<Vec2>| !c.is_empty()).unwrap_or(false),
            VertexAttribute::StaticOffset => false,
        }
    }
}

/// Mesh vertex attribute for GPU
#[derive(Debug, Clone)]
pub struct MeshVertexAttribute {
    pub name: String,
    pub num_components: u32,
    pub component_type: VertexComponentType,
    pub normalize: bool,
    pub offset: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum VertexComponentType {
    Float,
    HalfFloat,
    Double,
    Int,
    UnsignedInt,
    Short,
    UnsignedShort,
    Byte,
    UnsignedByte,
}

/// Helper to create common meshes
pub struct MeshBuilder;

impl MeshBuilder {
    /// Create a simple box mesh
    pub fn create_box(width: f32, height: f32, depth: f32) -> Arc<Mesh> {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let hd = depth / 2.0;

        let mesh = Arc::new(Mesh::new("Box"));

        let positions = vec![
            // Front face
            Vec3::new(-hw, -hh, hd),
            Vec3::new(hw, -hh, hd),
            Vec3::new(hw, hh, hd),
            Vec3::new(-hw, hh, hd),
            // Back face
            Vec3::new(hw, -hh, -hd),
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(-hw, hh, -hd),
            Vec3::new(hw, hh, -hd),
            // Top face
            Vec3::new(-hw, hh, hd),
            Vec3::new(hw, hh, hd),
            Vec3::new(hw, hh, -hd),
            Vec3::new(-hw, hh, -hd),
            // Bottom face
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(hw, -hh, -hd),
            Vec3::new(hw, -hh, hd),
            Vec3::new(-hw, -hh, hd),
            // Right face
            Vec3::new(hw, -hh, hd),
            Vec3::new(hw, -hh, -hd),
            Vec3::new(hw, hh, -hd),
            Vec3::new(hw, hh, hd),
            // Left face
            Vec3::new(-hw, -hh, -hd),
            Vec3::new(-hw, -hh, hd),
            Vec3::new(-hw, hh, hd),
            Vec3::new(-hw, hh, -hd),
        ];

        let normals = vec![
            // Front
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, 1.0),
            // Back
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 0.0, -1.0),
            // Top
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            // Bottom
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            // Right
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            // Left
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,   // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];

        mesh.set_positions(positions);
        mesh.set_normals(normals);
        mesh.set_indices(indices);

        mesh
    }

    /// Create a simple sphere mesh
    pub fn create_sphere(radius: f32, rings: usize, segments: usize) -> Arc<Mesh> {
        let mesh = Arc::new(Mesh::new("Sphere"));

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::new();

        let r_theta = std::f32::consts::PI / rings as f32;
        let r_phi = 2.0 * std::f32::consts::PI / segments as f32;

        for i in 0..=rings {
            let theta = i as f32 * r_theta;
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();

            for j in 0..=segments {
                let phi = j as f32 * r_phi;
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();

                let x = cos_phi * sin_theta;
                let y = cos_theta;
                let z = sin_phi * sin_theta;

                positions.push(Vec3::new(x * radius, y * radius, z * radius));
                normals.push(Vec3::new(x, y, z));
                tex_coords.push(Vec2::new(j as f32 / segments as f32, i as f32 / rings as f32));
            }
        }

        for i in 0..rings {
            for j in 0..segments {
                let a = i * (segments + 1) + j;
                let b = a + segments + 1;

                indices.push(a as u32);
                indices.push(b as u32);
                indices.push((a + 1) as u32);

                indices.push(b as u32);
                indices.push((b + 1) as u32);
                indices.push((a + 1) as u32);
            }
        }

        mesh.set_positions(positions);
        mesh.set_normals(normals);
        mesh.set_tex_coords(0, tex_coords);
        mesh.set_indices(indices);

        mesh
    }

    /// Create a plane mesh
    pub fn create_plane(width: f32, height: f32, segments_w: usize, segments_h: usize) -> Arc<Mesh> {
        let mesh = Arc::new(Mesh::new("Plane"));

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::new();

        let hw = width / 2.0;
        let hh = height / 2.0;
        let step_w = width / segments_w as f32;
        let step_h = height / segments_h as f32;

        for i in 0..=segments_h {
            for j in 0..=segments_w {
                let x = -hw + j as f32 * step_w;
                let z = -hh + i as f32 * step_h;

                positions.push(Vec3::new(x, 0.0, z));
                normals.push(Vec3::new(0.0, 1.0, 0.0));
                tex_coords.push(Vec2::new(j as f32 / segments_w as f32, i as f32 / segments_h as f32));
            }
        }

        for i in 0..segments_h {
            for j in 0..segments_w {
                let a = i * (segments_w + 1) + j;
                let b = a + segments_w + 1;

                indices.push(a as u32);
                indices.push(b as u32);
                indices.push((a + 1) as u32);

                indices.push(b as u32);
                indices.push((b + 1) as u32);
                indices.push((a + 1) as u32);
            }
        }

        mesh.set_positions(positions);
        mesh.set_normals(normals);
        mesh.set_tex_coords(0, tex_coords);
        mesh.set_indices(indices);

        mesh
    }
}
