// Geometry - Leaf node containing a mesh and material

use crate::math::{Mat4, Transform};
use crate::scene::{spatial::*, BoundingVolume, Mesh, Spatial};
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// Geometry - A leaf node in the scene graph that can be rendered
pub struct Geometry {
    id: Uuid,
    name: RwLock<String>,
    local_transform: RwLock<Transform>,
    world_transform: RwLock<Mat4>,
    enabled: RwLock<bool>,
    batched: RwLock<bool>,
    bounding: RwLock<Option<BoundingVolume>>,
    parent: RwLock<Option<Arc<dyn NodeTrait>>>,
    mesh: RwLock<Option<Arc<Mesh>>>,
    material: RwLock<Option<Arc<dyn Material>>>,
}

impl Geometry {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: RwLock::new(name.to_string()),
            local_transform: RwLock::new(Transform::identity()),
            world_transform: RwLock::new(Mat4::IDENTITY),
            enabled: RwLock::new(true),
            batched: RwLock::new(false),
            bounding: RwLock::new(None),
            parent: RwLock::new(None),
            mesh: RwLock::new(None),
            material: RwLock::new(None),
        }
    }

    pub fn new_with_mesh(name: &str, mesh: Arc<Mesh>) -> Self {
        let mut geom = Self::new(name);
        *geom.mesh.write() = Some(mesh);
        geom
    }

    /// Set the mesh
    pub fn set_mesh(&self, mesh: Arc<Mesh>) {
        *self.mesh.write() = Some(mesh);
    }

    /// Get the mesh
    pub fn get_mesh(&self) -> Option<Arc<Mesh>> {
        self.mesh.read().clone()
    }

    /// Set the material
    pub fn set_material(&self, material: Arc<dyn Material>) {
        *self.material.write() = Some(material);
    }

    /// Get the material
    pub fn get_material(&self) -> Option<Arc<dyn Material>> {
        self.material.read().clone()
    }

    /// Check if this geometry is renderable
    pub fn is_renderable(&self) -> bool {
        self.mesh.read().is_some() && self.is_enabled()
    }

    /// Update bounding volume from mesh
    pub fn update_bound(&self) {
        if let Some(mesh) = self.mesh.read().as_ref() {
            if let Some(bounding) = mesh.get_bounding() {
                *self.bounding.write() = Some(bounding.transform(&self.get_world_transform()));
            }
        }
    }
}

impl Spatial for Geometry {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> String {
        self.name.read().clone()
    }

    fn set_name(&mut self, name: &str) {
        *self.name.write() = name.to_string();
    }

    fn get_local_transform(&self) -> Transform {
        *self.local_transform.read()
    }

    fn set_local_transform(&mut self, transform: Transform) {
        *self.local_transform.write() = transform;
    }

    fn get_world_transform(&self) -> Mat4 {
        *self.world_transform.read()
    }

    fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    fn set_enabled(&mut self, enabled: bool) {
        *self.enabled.write() = enabled;
    }

    fn is_batched(&self) -> bool {
        *self.batched.read()
    }

    fn set_batched(&mut self, batched: bool) {
        *self.batched.write() = batched;
    }

    fn get_bounding(&self) -> Option<BoundingVolume> {
        self.bounding.read().clone()
    }

    fn set_bounding(&mut self, bounding: BoundingVolume) {
        *self.bounding.write() = Some(bounding);
    }

    fn get_parent(&self) -> Option<Arc<dyn NodeTrait>> {
        self.parent.read().clone()
    }

    fn set_parent(&mut self, parent: Option<Arc<dyn NodeTrait>>) {
        // Handled via special method
    }

    fn update_world_transform(&self, parent_transform: Mat4) {
        let local = self.get_local_transform();
        let world = parent_transform * local.to_matrix();
        *self.world_transform.write() = world;

        // Update bounding
        self.update_bound();
    }
}

/// Trait for materials
pub trait Material: Send + Sync {
    /// Get the material name
    fn get_name(&self) -> &str;

    /// Check if this is a PBR material
    fn is_pbr(&self) -> bool;

    /// Get the shader name
    fn get_shader_name(&self) -> &str;

    /// Get a parameter value
    fn get_param(&self, name: &str) -> Option<MaterialParam>;

    /// Set a parameter value
    fn set_param(&mut self, name: &str, value: MaterialParam);

    /// Clone the material
    fn clone_box(&self) -> Box<dyn Material>;
}

/// Material parameter types
#[derive(Clone)]
pub enum MaterialParam {
    Float(f32),
    Vec2(glam::Vec2),
    Vec3(glam::Vec3),
    Vec4(glam::Vec4),
    Texture(Arc<dyn Texture>), // Placeholder for texture
}

impl std::fmt::Debug for MaterialParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float(arg0) => f.debug_tuple("Float").field(arg0).finish(),
            Self::Vec2(arg0) => f.debug_tuple("Vec2").field(arg0).finish(),
            Self::Vec3(arg0) => f.debug_tuple("Vec3").field(arg0).finish(),
            Self::Vec4(arg0) => f.debug_tuple("Vec4").field(arg0).finish(),
            Self::Texture(_) => f.debug_tuple("Texture").finish(),
        }
    }
}

impl MaterialParam {
    pub fn as_float(&self) -> Option<f32> {
        if let MaterialParam::Float(v) = self { Some(*v) } else { None }
    }

    pub fn as_vec3(&self) -> Option<glam::Vec3> {
        if let MaterialParam::Vec3(v) = self { Some(*v) } else { None }
    }
}

/// Placeholder for texture
pub trait Texture: Send + Sync {}
