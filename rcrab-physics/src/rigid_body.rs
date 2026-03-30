// Rigid body - stub implementation

use rcrab_core::math::{Vec3, Quat};
use uuid::Uuid;

/// Rigid body type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RigidBodyType {
    Dynamic,
    Static,
    KinematicPositionBased,
    KinematicVelocityBased,
}

/// Rigid body wrapper (stub)
pub struct RigidBody {
    id: Uuid,
    body_type: RigidBodyType,
    position: Vec3,
    rotation: Quat,
    linear_velocity: Vec3,
    angular_velocity: Vec3,
}

impl RigidBody {
    pub fn new(body_type: RigidBodyType) -> Self {
        tracing::warn!("Physics disabled - stub implementation");
        Self {
            id: Uuid::new_v4(),
            body_type,
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            linear_velocity: Vec3::ZERO,
            angular_velocity: Vec3::ZERO,
        }
    }

    pub fn dynamic() -> Self {
        Self::new(RigidBodyType::Dynamic)
    }

    pub fn static_body() -> Self {
        Self::new(RigidBodyType::Static)
    }

    pub fn kinematic() -> Self {
        Self::new(RigidBodyType::KinematicPositionBased)
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn get_rotation(&self) -> Quat {
        self.rotation
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn get_linear_velocity(&self) -> Vec3 {
        self.linear_velocity
    }

    pub fn set_linear_velocity(&mut self, velocity: Vec3) {
        self.linear_velocity = velocity;
    }

    pub fn get_angular_velocity(&self) -> Vec3 {
        self.angular_velocity
    }

    pub fn set_angular_velocity(&mut self, velocity: Vec3) {
        self.angular_velocity = velocity;
    }

    pub fn apply_impulse(&mut self, _impulse: Vec3) {}
    pub fn apply_angular_impulse(&mut self, _impulse: Vec3) {}
    pub fn apply_force(&mut self, _force: Vec3) {}
    pub fn apply_torque(&mut self, _torque: Vec3) {}

    pub fn get_mass(&self) -> f32 {
        1.0
    }

    pub fn set_mass(&mut self, _mass: f32) {}

    pub fn set_gravity_enabled(&mut self, _enabled: bool) {}
    pub fn is_gravity_enabled(&self) -> bool { true }

    pub fn set_linear_damping(&mut self, _damping: f32) {}
    pub fn set_angular_damping(&mut self, _damping: f32) {}

    pub fn set_kinematic_target(&mut self, position: Vec3, rotation: Quat) {
        self.position = position;
        self.rotation = rotation;
    }

    pub fn is_dynamic(&self) -> bool {
        self.body_type == RigidBodyType::Dynamic
    }

    pub fn is_fixed(&self) -> bool {
        self.body_type == RigidBodyType::Static
    }

    pub fn is_kinematic(&self) -> bool {
        matches!(self.body_type, RigidBodyType::KinematicPositionBased | RigidBodyType::KinematicVelocityBased)
    }

    pub fn get_body_type(&self) -> RigidBodyType {
        self.body_type
    }
}

/// Extension trait for Spatial to integrate with physics (stub)
pub trait PhysicsBody {
    fn get_rigid_body(&self) -> Option<&RigidBody>;
    fn set_rigid_body(&self, _body: RigidBody);
}
