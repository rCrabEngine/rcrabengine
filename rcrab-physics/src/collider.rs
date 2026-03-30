// Collider - stub implementation

use rcrab_core::math::Vec3;
use uuid::Uuid;

/// Collision shape types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionShapeType {
    Ball,
    Cuboid,
    Capsule,
    Cylinder,
    Cone,
    ConvexHull,
    Compound,
    Heightfield,
    Trimesh,
}

/// Collision shape wrapper (stub)
pub struct CollisionShape {
    shape_type: CollisionShapeType,
}

impl CollisionShape {
    pub fn ball(_radius: f32) -> Self {
        Self { shape_type: CollisionShapeType::Ball }
    }

    pub fn cuboid(_half_extents: Vec3) -> Self {
        Self { shape_type: CollisionShapeType::Cuboid }
    }

    pub fn capsule(_radius: f32, _half_height: f32) -> Self {
        Self { shape_type: CollisionShapeType::Capsule }
    }

    pub fn cylinder(_half_height: f32, _radius: f32) -> Self {
        Self { shape_type: CollisionShapeType::Cylinder }
    }

    pub fn cone(_half_height: f32, _radius: f32) -> Self {
        Self { shape_type: CollisionShapeType::Cone }
    }

    pub fn get_shape_type(&self) -> CollisionShapeType {
        self.shape_type
    }
}

/// Collider (stub)
pub struct Collider {
    id: Uuid,
    shape: CollisionShape,
    friction: f32,
    restitution: f32,
    sensor: bool,
}

impl Collider {
    pub fn new(shape: CollisionShape) -> Self {
        Self {
            id: Uuid::new_v4(),
            shape,
            friction: 0.5,
            restitution: 0.5,
            sensor: false,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_friction(&mut self, friction: f32) {
        self.friction = friction;
    }

    pub fn get_friction(&self) -> f32 {
        self.friction
    }

    pub fn set_restitution(&mut self, restitution: f32) {
        self.restitution = restitution;
    }

    pub fn get_restitution(&self) -> f32 {
        self.restitution
    }

    pub fn set_collision_groups(&mut self, _groups: u32) {}

    pub fn set_sensor(&mut self, sensor: bool) {
        self.sensor = sensor;
    }

    pub fn is_sensor(&self) -> bool {
        self.sensor
    }

    pub fn set_translation(&mut self, _translation: Vec3) {}
    pub fn set_rotation(&mut self, _rotation: rcrab_core::math::Quat) {}
}

/// Helper to create common colliders (stub)
pub struct ColliderBuilder;

impl ColliderBuilder {
    pub fn ball(radius: f32) -> Collider {
        Collider::new(CollisionShape::ball(radius))
    }

    pub fn box_fn(width: f32, height: f32, depth: f32) -> Collider {
        Collider::new(CollisionShape::cuboid(Vec3::new(width / 2.0, height / 2.0, depth / 2.0)))
    }

    pub fn capsule(radius: f32, height: f32) -> Collider {
        Collider::new(CollisionShape::capsule(radius, height / 2.0 - radius))
    }

    pub fn cylinder(radius: f32, height: f32) -> Collider {
        Collider::new(CollisionShape::cylinder(height / 2.0, radius))
    }

    pub fn cone(radius: f32, height: f32) -> Collider {
        Collider::new(CollisionShape::cone(height / 2.0, radius))
    }
}
