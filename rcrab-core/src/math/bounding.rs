// Bounding volumes for culling and collision

use glam::{Mat4, Vec3};
use serde::{Deserialize, Serialize};

/// Type of bounding volume
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoundingType {
    Sphere,
    Box,
    None,
}

/// Axis-Aligned Bounding Box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingBox {
    pub center: Vec3,
    pub extents: Vec3, // half-extents
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            extents: Vec3::ZERO,
        }
    }
}

impl BoundingBox {
    pub fn new(center: Vec3, extents: Vec3) -> Self {
        Self { center, extents }
    }

    pub fn from_min_max(min: Vec3, max: Vec3) -> Self {
        let center = (min + max) * 0.5;
        let extents = (max - min) * 0.5;
        Self { center, extents }
    }

    pub fn min(&self) -> Vec3 {
        self.center - self.extents
    }

    pub fn max(&self) -> Vec3 {
        self.center + self.extents
    }

    pub fn transform(&self, world_matrix: &Mat4) -> BoundingBox {
        // For simplicity, we'll just transform the center and use the max extent
        let new_center = world_matrix.transform_point3(self.center);

        // Extract scale from matrix for accurate bounding
        let scale = Vec3::new(
            Vec3::new(world_matrix.x_axis.x, world_matrix.x_axis.y, world_matrix.x_axis.z).length(),
            Vec3::new(world_matrix.y_axis.x, world_matrix.y_axis.y, world_matrix.y_axis.z).length(),
            Vec3::new(world_matrix.z_axis.x, world_matrix.z_axis.y, world_matrix.z_axis.z).length(),
        );

        BoundingBox {
            center: new_center,
            extents: self.extents * scale,
        }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        let diff = (point - self.center).abs();
        diff.x <= self.extents.x && diff.y <= self.extents.y && diff.z <= self.extents.z
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        let diff = (other.center - self.center).abs();
        diff.x <= self.extents.x + other.extents.x
            && diff.y <= self.extents.y + other.extents.y
            && diff.z <= self.extents.z + other.extents.z
    }
}

/// Bounding sphere
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Default for BoundingSphere {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            radius: 0.0,
        }
    }
}

impl BoundingSphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn transform(&self, world_matrix: &Mat4) -> BoundingSphere {
        let new_center = world_matrix.transform_point3(self.center);

        // Use the maximum scale to expand the radius
        let scale = Vec3::new(
            Vec3::new(world_matrix.x_axis.x, world_matrix.x_axis.y, world_matrix.x_axis.z).length(),
            Vec3::new(world_matrix.y_axis.x, world_matrix.y_axis.y, world_matrix.y_axis.z).length(),
            Vec3::new(world_matrix.z_axis.x, world_matrix.z_axis.y, world_matrix.z_axis.z).length(),
        );

        BoundingSphere {
            center: new_center,
            radius: self.radius * scale.max_element(),
        }
    }

    pub fn contains_point(&self, point: Vec3) -> bool {
        (point - self.center).length_squared() <= self.radius * self.radius
    }

    pub fn intersects(&self, other: &BoundingSphere) -> bool {
        let diff = other.center - self.center;
        let dist_sq = diff.length_squared();
        let radius_sum = self.radius + other.radius;
        dist_sq <= radius_sum * radius_sum
    }

    pub fn intersects_box(&self, other: &BoundingBox) -> bool {
        // Find the point on the box closest to the sphere center
        let closest = Vec3::new(
            self.center.x.clamp(other.min().x, other.max().x),
            self.center.y.clamp(other.min().y, other.max().y),
            self.center.z.clamp(other.min().z, other.max().z),
        );

        let diff = closest - self.center;
        diff.length_squared() <= self.radius * self.radius
    }
}

/// Combined bounding volume that can be either sphere or box
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BoundingVolume {
    Sphere(BoundingSphere),
    Box(BoundingBox),
}

impl Default for BoundingVolume {
    fn default() -> Self {
        BoundingVolume::Sphere(BoundingSphere::default())
    }
}

impl BoundingVolume {
    pub fn get_type(&self) -> BoundingType {
        match self {
            BoundingVolume::Sphere(_) => BoundingType::Sphere,
            BoundingVolume::Box(_) => BoundingType::Box,
        }
    }

    pub fn transform(&self, world_matrix: &Mat4) -> BoundingVolume {
        match self {
            BoundingVolume::Sphere(s) => BoundingVolume::Sphere(s.transform(world_matrix)),
            BoundingVolume::Box(b) => BoundingVolume::Box(b.transform(world_matrix)),
        }
    }
}
