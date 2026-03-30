// Audio listener

use rcrab_core::math::Vec3;

/// Audio listener - represents the ear position in 3D space
pub struct AudioListener {
    position: Vec3,
    forward: Vec3,
    up: Vec3,
    velocity: Vec3,
}

impl AudioListener {
    /// Create a new audio listener
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            forward: Vec3::NEG_Z,
            up: Vec3::Y,
            velocity: Vec3::ZERO,
        }
    }

    /// Get position
    pub fn get_position(&self) -> Vec3 {
        self.position
    }

    /// Set position
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Get forward direction
    pub fn get_forward(&self) -> Vec3 {
        self.forward
    }

    /// Set forward direction
    pub fn set_forward(&mut self, forward: Vec3) {
        self.forward = forward.normalize();
    }

    /// Get up direction
    pub fn get_up(&self) -> Vec3 {
        self.up
    }

    /// Set up direction
    pub fn set_up(&mut self, up: Vec3) {
        self.up = up.normalize();
    }

    /// Get velocity
    pub fn get_velocity(&self) -> Vec3 {
        self.velocity
    }

    /// Set velocity (for doppler effect)
    pub fn set_velocity(&mut self, velocity: Vec3) {
        self.velocity = velocity;
    }

    /// Set orientation (forward and up)
    pub fn set_orientation(&mut self, forward: Vec3, up: Vec3) {
        self.forward = forward.normalize();
        self.up = up.normalize();
    }

    /// Update based on camera position
    pub fn set_from_camera(&mut self, position: Vec3, forward: Vec3, up: Vec3) {
        self.position = position;
        self.forward = forward.normalize();
        self.up = up.normalize();
    }
}

impl Default for AudioListener {
    fn default() -> Self {
        Self::new()
    }
}
