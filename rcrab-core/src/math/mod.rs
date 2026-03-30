// Math types - Re-export from glam with game-engine specific helpers

pub use glam::{Mat4, Quat, Vec2, Vec3, Vec3A, Vec4};

pub mod curve;
pub mod bounding;

pub use bounding::{BoundingBox, BoundingSphere, BoundingVolume, BoundingType};

use serde::{Deserialize, Serialize};

/// A 3D transformation (position, rotation, scale)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_position(pos: Vec3) -> Self {
        Self {
            position: pos,
            ..Default::default()
        }
    }

    pub fn from_rotation(rot: Quat) -> Self {
        Self {
            rotation: rot,
            ..Default::default()
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }

    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position: translation,
            rotation,
            scale,
        }
    }

    /// Combine this transform with another (this * other)
    pub fn combine(&self, other: &Transform) -> Self {
        let combined_scale = self.scale * other.scale;
        let combined_rotation = self.rotation * other.rotation;
        let combined_position = self.position + self.rotation * (self.scale * other.position);

        Self {
            position: combined_position,
            rotation: combined_rotation,
            scale: combined_scale,
        }
    }

    /// Get the transformation matrix
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    /// Invert the transform
    pub fn invert(&self) -> Transform {
        let inv_scale = Vec3::ONE / self.scale;
        let inv_rot = self.rotation.inverse();
        let inv_pos = -(inv_rot * (inv_scale * self.position));

        Transform {
            position: inv_pos,
            rotation: inv_rot,
            scale: inv_scale,
        }
    }

    /// Transform a point
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * (self.scale * point) + self.position
    }

    /// Transform a direction vector
    pub fn transform_direction(&self, dir: Vec3) -> Vec3 {
        self.rotation * dir
    }

    /// Look at target (for camera/forward orientation)
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let direction = (target - self.position).normalize();
        self.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction);
        // Correct for up vector
        let right = direction.cross(up).normalize();
        let new_up = right.cross(direction);
        let rotation_matrix = Mat4::from_cols(
            right.extend(0.0),
            new_up.extend(0.0),
            (-direction).extend(0.0),
            Vec4::ZERO
        );
        self.rotation = Quat::from_mat4(&rotation_matrix);
    }

    /// Get forward direction (negative Z)
    pub fn forward(&self) -> Vec3 {
        -self.rotation * Vec3::Z
    }

    /// Get backward direction
    pub fn backward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    /// Get up direction
    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    /// Get down direction
    pub fn down(&self) -> Vec3 {
        -self.rotation * Vec3::Y
    }

    /// Get right direction
    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    /// Get left direction
    pub fn left(&self) -> Vec3 {
        -self.rotation * Vec3::X
    }
}

/// Convert between glam types and jME3-style types
pub mod convert {
    use super::*;
    use glam::{Mat4, Vec3, Vec4};

    pub fn vec3_to_glam(v: &Vec3) -> Vec3 {
        *v
    }

    pub fn glam_to_vec3(v: Vec3) -> Vec3 {
        v
    }

    pub fn mat4_to_glam(m: &Mat4) -> Mat4 {
        *m
    }

    pub fn glam_to_mat4(m: Mat4) -> Mat4 {
        m
    }
}
