// Physics space - stub implementation

use rcrab_core::math::Vec3;

/// Physics world configuration (stub)
pub struct PhysicsWorld;

impl PhysicsWorld {
    pub fn new(_gravity: Vec3) -> Self {
        tracing::warn!("Physics disabled - stub implementation");
        Self
    }

    pub fn with_default_gravity() -> Self {
        Self
    }

    pub fn get_gravity(&self) -> Vec3 {
        Vec3::new(0.0, -9.81, 0.0)
    }

    pub fn set_gravity(&self, _gravity: Vec3) {}

    pub fn step(&self) {
        tracing::warn!("Physics disabled - stub implementation");
    }

    pub fn num_bodies(&self) -> usize { 0 }
    pub fn num_colliders(&self) -> usize { 0 }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::with_default_gravity()
    }
}

/// Physics space - high-level interface (stub)
pub struct PhysicsSpace {
    gravity: Vec3,
}

impl PhysicsSpace {
    pub fn new(gravity: Vec3) -> Self {
        tracing::warn!("Physics disabled - stub implementation");
        Self { gravity }
    }

    pub fn with_default_gravity() -> Self {
        Self::new(Vec3::new(0.0, -9.81, 0.0))
    }

    pub fn get_world(&self) -> &PhysicsWorld {
        // Return a static stub
        static WORLD: PhysicsWorld = PhysicsWorld;
        &WORLD
    }

    pub fn register_body(&self, _body: &crate::RigidBody) {}
    pub fn unregister_body(&self, _id: uuid::Uuid) {}
    pub fn get_body_handle(&self, _id: uuid::Uuid) -> Option<crate::RigidBody> { None }

    pub fn update(&self) {
        tracing::warn!("Physics disabled - stub implementation");
    }

    pub fn get_gravity(&self) -> Vec3 {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec3) {
        self.gravity = gravity;
    }

    pub fn ray_cast(&self, _origin: Vec3, _direction: Vec3, _max_toi: f32) -> Option<RayCastHit> {
        None
    }

    pub fn num_bodies(&self) -> usize { 0 }
}

impl Default for PhysicsSpace {
    fn default() -> Self {
        Self::with_default_gravity()
    }
}

/// Ray cast hit result (stub)
pub struct RayCastHit {
    pub handle: (),
    pub collider_handle: (),
    pub toi: f32,
    pub normal: Vec3,
}
