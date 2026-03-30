// Spatial - Base trait for scene graph objects

use crate::math::{Mat4, Transform, Vec3};
use crate::scene::BoundingVolume;
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// Base trait for all scene graph objects
pub trait Spatial: Send + Sync {
    /// Get the unique ID of this spatial
    fn get_id(&self) -> Uuid;

    /// Get the name of this spatial
    fn get_name(&self) -> String;

    /// Set the name
    fn set_name(&mut self, name: &str);

    /// Get the local transform
    fn get_local_transform(&self) -> Transform;

    /// Set the local transform
    fn set_local_transform(&mut self, transform: Transform);

    /// Get the world transform (computed from parent chain)
    fn get_world_transform(&self) -> Mat4;

    /// Check if this spatial is enabled
    fn is_enabled(&self) -> bool;

    /// Set enabled state
    fn set_enabled(&mut self, enabled: bool);

    /// Check if this spatial is batched
    fn is_batched(&self) -> bool;

    /// Set batched state
    fn set_batched(&mut self, batched: bool);

    /// Get the bounding volume
    fn get_bounding(&self) -> Option<BoundingVolume>;

    /// Set the bounding volume
    fn set_bounding(&mut self, bounding: BoundingVolume);

    /// Get the parent node (if any)
    fn get_parent(&self) -> Option<Arc<dyn NodeTrait>>;

    /// Set the parent node
    fn set_parent(&mut self, parent: Option<Arc<dyn NodeTrait>>);

    /// Update the world transform
    fn update_world_transform(&self, parent_transform: Mat4);

    /// Cast to node if possible (default returns None)
    fn as_node(&self) -> Option<&dyn NodeTrait> {
        None
    }

    /// Cast to geometry if possible (default returns None)
    fn as_geometry(&self) -> Option<&dyn GeometryTrait> {
        None
    }

    /// Get the world position
    fn get_world_position(&self) -> Vec3 {
        self.get_world_transform().w_axis.truncate()
    }

    /// Move by offset
    fn move_by(&mut self, offset: Vec3) {
        let mut transform = self.get_local_transform();
        transform.position += offset;
        self.set_local_transform(transform);
    }

    /// Set position
    fn set_position(&mut self, position: Vec3) {
        let mut transform = self.get_local_transform();
        transform.position = position;
        self.set_local_transform(transform);
    }

    /// Set rotation
    fn set_rotation(&mut self, rotation: crate::math::Quat) {
        let mut transform = self.get_local_transform();
        transform.rotation = rotation;
        self.set_local_transform(transform);
    }

    /// Set scale
    fn set_scale(&mut self, scale: Vec3) {
        let mut transform = self.get_local_transform();
        transform.scale = scale;
        self.set_local_transform(transform);
    }

    /// Look at a target
    fn look_at(&mut self, target: Vec3, up: Vec3) {
        let mut transform = self.get_local_transform();
        transform.look_at(target, up);
        self.set_local_transform(transform);
    }
}

/// Trait for nodes that can have children
pub trait NodeTrait: Send + Sync {
    /// Get the number of children
    fn get_num_children(&self) -> usize;

    /// Get all children
    fn get_children(&self) -> Vec<Arc<dyn Spatial>>;

    /// Add a child
    fn add_child(&self, child: Arc<dyn Spatial>);

    /// Remove a child by index
    fn remove_child(&self, index: usize) -> Option<Arc<dyn Spatial>>;

    /// Remove a child by reference
    fn remove_child_spatial(&self, child: &dyn Spatial) -> bool;

    /// Get a child by index
    fn get_child(&self, index: usize) -> Option<Arc<dyn Spatial>>;

    /// Get a child by name
    fn get_child_by_name(&self, name: &str) -> Option<Arc<dyn Spatial>>;

    /// Detach all children
    fn detach_all_children(&self);

    /// Update all children
    fn update_children(&self, parent_transform: Mat4);
}

/// Trait for geometry objects
pub trait GeometryTrait: Send + Sync {
    fn get_mesh(&self) -> Option<Arc<crate::scene::Mesh>>;
}

/// Extension trait for working with spatial objects
pub trait SpatialExt {
    fn as_node(&self) -> Option<&dyn NodeTrait>;
}

impl SpatialExt for Arc<dyn Spatial> {
    fn as_node(&self) -> Option<&dyn NodeTrait> {
        // This would require downcasting, which is handled elsewhere
        None
    }
}

/// Helper for creating unique names
pub fn generate_name() -> String {
    format!("Spatial_{}", Uuid::new_v4())
}
