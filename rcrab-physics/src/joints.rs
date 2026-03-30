// Physics joints - stub implementation

use rcrab_core::math::Vec3;
use uuid::Uuid;

/// Joint data types (stub)
#[derive(Debug, Clone, Copy)]
pub enum JointData {
    Ball(BallJointData),
    Hinge(HingeJointData),
    Prismatic(PrismaticJointData),
    Fixed(FixedJointData),
}

/// Ball joint (spherical joint)
#[derive(Debug, Clone, Copy)]
pub struct BallJointData {
    pub anchor1: Vec3,
    pub anchor2: Vec3,
}

impl BallJointData {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self {
        Self { anchor1, anchor2 }
    }
}

/// Hinge joint
#[derive(Debug, Clone, Copy)]
pub struct HingeJointData {
    pub anchor1: Vec3,
    pub anchor2: Vec3,
    pub axis1: Vec3,
    pub axis2: Vec3,
    pub limits: Option<(f32, f32)>,
}

impl HingeJointData {
    pub fn new(anchor1: Vec3, anchor2: Vec3, axis1: Vec3, axis2: Vec3) -> Self {
        Self {
            anchor1,
            anchor2,
            axis1,
            axis2,
            limits: None,
        }
    }

    pub fn with_limits(mut self, min: f32, max: f32) -> Self {
        self.limits = Some((min, max));
        self
    }
}

/// Prismatic joint (slider)
#[derive(Debug, Clone, Copy)]
pub struct PrismaticJointData {
    pub anchor1: Vec3,
    pub anchor2: Vec3,
    pub axis1: Vec3,
    pub axis2: Vec3,
    pub limits: Option<(f32, f32)>,
}

impl PrismaticJointData {
    pub fn new(anchor1: Vec3, anchor2: Vec3, axis1: Vec3, axis2: Vec3) -> Self {
        Self {
            anchor1,
            anchor2,
            axis1,
            axis2,
            limits: None,
        }
    }

    pub fn with_limits(mut self, min: f32, max: f32) -> Self {
        self.limits = Some((min, max));
        self
    }
}

/// Fixed joint data
#[derive(Debug, Clone, Copy)]
pub struct FixedJointData {
    pub anchor1: Vec3,
    pub anchor2: Vec3,
}

impl FixedJointData {
    pub fn new(anchor1: Vec3, anchor2: Vec3) -> Self {
        Self { anchor1, anchor2 }
    }
}

/// Physics joint wrapper (stub)
pub struct PhysicsJoint {
    id: Uuid,
    joint_type: JointData,
}

impl PhysicsJoint {
    pub fn ball(data: BallJointData) -> Self {
        tracing::warn!("Physics disabled - stub implementation");
        Self {
            id: Uuid::new_v4(),
            joint_type: JointData::Ball(data),
        }
    }

    pub fn hinge(data: HingeJointData) -> Self {
        Self {
            id: Uuid::new_v4(),
            joint_type: JointData::Hinge(data),
        }
    }

    pub fn prismatic(data: PrismaticJointData) -> Self {
        Self {
            id: Uuid::new_v4(),
            joint_type: JointData::Prismatic(data),
        }
    }

    pub fn fixed(anchor1: Vec3, anchor2: Vec3) -> Self {
        Self {
            id: Uuid::new_v4(),
            joint_type: JointData::Fixed(FixedJointData::new(anchor1, anchor2)),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
