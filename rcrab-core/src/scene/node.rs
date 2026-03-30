// Node - Container for child spatials

use crate::math::{Mat4, Transform, Vec3};
use crate::scene::{spatial::*, BoundingVolume, Spatial};
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// A node is a container for child spatial objects
pub struct Node {
    id: Uuid,
    name: RwLock<String>,
    local_transform: RwLock<Transform>,
    world_transform: RwLock<Mat4>,
    enabled: RwLock<bool>,
    batched: RwLock<bool>,
    bounding: RwLock<Option<BoundingVolume>>,
    parent: RwLock<Option<Arc<dyn NodeTrait>>>,
    children: RwLock<Vec<Arc<dyn Spatial>>>,
}

impl Node {
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
            children: RwLock::new(Vec::new()),
        }
    }

    pub fn new_with_id(name: &str, id: Uuid) -> Self {
        Self {
            id,
            name: RwLock::new(name.to_string()),
            local_transform: RwLock::new(Transform::identity()),
            world_transform: RwLock::new(Mat4::IDENTITY),
            enabled: RwLock::new(true),
            batched: RwLock::new(false),
            bounding: RwLock::new(None),
            parent: RwLock::new(None),
            children: RwLock::new(Vec::new()),
        }
    }

    /// Attach a child to this node
    pub fn attach_child(&self, child: Arc<dyn Spatial>) {
        // Add to children list
        self.children.write().push(child.clone());
    }

    /// Detach a child at index
    pub fn detach_child_at(&self, index: usize) -> Option<Arc<dyn Spatial>> {
        let mut children = self.children.write();
        if index < children.len() {
            Some(children.remove(index))
        } else {
            None
        }
    }

    /// Detach a specific child
    pub fn detach_child(&self, child: &dyn Spatial) -> bool {
        let mut children = self.children.write();
        if let Some(pos) = children.iter().position(|c| c.get_id() == child.get_id()) {
            children.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get the number of children
    pub fn get_quantity(&self) -> usize {
        self.children.read().len()
    }

    /// Get all children as a slice
    pub fn get_children_slice(&self) -> Vec<Arc<dyn Spatial>> {
        self.children.read().clone()
    }

    /// Get child at index
    pub fn get_child(&self, index: usize) -> Option<Arc<dyn Spatial>> {
        self.children.read().get(index).cloned()
    }

    /// Find child by name
    pub fn find_child(&self, name: &str) -> Option<Arc<dyn Spatial>> {
        let children = self.children.read();
        for child in children.iter() {
            if child.get_name() == name {
                return Some(child.clone());
            }
        }
        None
    }

    /// Detach all children
    pub fn clear(&self) {
        self.children.write().clear();
    }

    /// Get all geometric children (non-node children)
    pub fn get_geometry(&self) -> Vec<Arc<dyn Spatial>> {
        self.children
            .read()
            .iter()
            .filter(|c| !c.as_node().is_some())
            .cloned()
            .collect()
    }

    /// Get all child nodes
    pub fn get_nodes(&self) -> Vec<Arc<dyn Spatial>> {
        self.children
            .read()
            .iter()
            .filter(|c| c.as_node().is_some())
            .cloned()
            .collect()
    }
}

impl Node {
    /// Cast to node trait
    pub fn as_node(&self) -> &dyn NodeTrait {
        self
    }
}

impl Spatial for Node {
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
        // This needs to be handled specially since we can't mutably borrow in a setter that returns
    }

    fn update_world_transform(&self, parent_transform: Mat4) {
        let local = self.get_local_transform();
        let world = parent_transform * local.to_matrix();
        *self.world_transform.write() = world;

        // Update all children
        self.update_children(world);
    }

    fn as_node(&self) -> Option<&dyn NodeTrait> {
        Some(self)
    }
}

impl NodeTrait for Node {
    fn get_num_children(&self) -> usize {
        self.children.read().len()
    }

    fn get_children(&self) -> Vec<Arc<dyn Spatial>> {
        self.children.read().clone()
    }

    fn add_child(&self, child: Arc<dyn Spatial>) {
        self.attach_child(child);
    }

    fn remove_child(&self, index: usize) -> Option<Arc<dyn Spatial>> {
        self.detach_child_at(index)
    }

    fn remove_child_spatial(&self, child: &dyn Spatial) -> bool {
        self.detach_child(child)
    }

    fn get_child(&self, index: usize) -> Option<Arc<dyn Spatial>> {
        self.get_child(index)
    }

    fn get_child_by_name(&self, name: &str) -> Option<Arc<dyn Spatial>> {
        self.find_child(name)
    }

    fn detach_all_children(&self) {
        self.clear();
    }

    fn update_children(&self, parent_transform: Mat4) {
        let children = self.children.read();
        for child in children.iter() {
            child.update_world_transform(parent_transform);
        }
    }
}
