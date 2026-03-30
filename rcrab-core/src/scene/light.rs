// Light - Light sources for the scene

use crate::math::{Mat4, Transform, Vec3, Vec4};
use parking_lot::RwLock;
use std::sync::Arc;
use uuid::Uuid;

/// Light type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Directional,
    Point,
    Spot,
    Ambient,
    Probe,
}

/// Base light
pub struct Light {
    id: Uuid,
    name: RwLock<String>,
    light_type: RwLock<LightType>,
    color: RwLock<Vec4>,
    intensity: RwLock<f32>,
    enabled: RwLock<bool>,
    position: RwLock<Vec3>,
    direction: RwLock<Vec3>,
    // Attenuation for point/spot lights
    constant_attenuation: RwLock<f32>,
    linear_attenuation: RwLock<f32>,
    quadratic_attenuation: RwLock<f32>,
    // Spot light properties
    spot_inner_angle: RwLock<f32>,
    spot_outer_angle: RwLock<f32>,
    // Shadow properties
    casts_shadows: RwLock<bool>,
    shadow_distance: RwLock<f32>,
    shadow_intensity: RwLock<f32>,
}

impl Light {
    pub fn new(name: &str, light_type: LightType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: RwLock::new(name.to_string()),
            light_type: RwLock::new(light_type),
            color: RwLock::new(Vec4::new(1.0, 1.0, 1.0, 1.0)),
            intensity: RwLock::new(1.0),
            enabled: RwLock::new(true),
            position: RwLock::new(Vec3::ZERO),
            direction: RwLock::new(Vec3::NEG_Z),
            constant_attenuation: RwLock::new(1.0),
            linear_attenuation: RwLock::new(0.0),
            quadratic_attenuation: RwLock::new(0.0),
            spot_inner_angle: RwLock::new(std::f32::consts::PI / 6.0),
            spot_outer_angle: RwLock::new(std::f32::consts::PI / 3.0),
            casts_shadows: RwLock::new(false),
            shadow_distance: RwLock::new(100.0),
            shadow_intensity: RwLock::new(0.5),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_name(&self) -> String {
        self.name.read().clone()
    }

    pub fn set_name(&self, name: &str) {
        *self.name.write() = name.to_string();
    }

    pub fn get_type(&self) -> LightType {
        *self.light_type.read()
    }

    pub fn get_color(&self) -> Vec4 {
        *self.color.read()
    }

    pub fn set_color(&self, color: Vec4) {
        *self.color.write() = color;
    }

    pub fn set_color_rgb(&self, r: f32, g: f32, b: f32) {
        *self.color.write() = Vec4::new(r, g, b, 1.0);
    }

    pub fn get_intensity(&self) -> f32 {
        *self.intensity.read()
    }

    pub fn set_intensity(&self, intensity: f32) {
        *self.intensity.write() = intensity;
    }

    pub fn is_enabled(&self) -> bool {
        *self.enabled.read()
    }

    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write() = enabled;
    }

    pub fn get_position(&self) -> Vec3 {
        *self.position.read()
    }

    pub fn set_position(&self, position: Vec3) {
        *self.position.write() = position;
    }

    pub fn get_direction(&self) -> Vec3 {
        *self.direction.read()
    }

    pub fn set_direction(&self, direction: Vec3) {
        *self.direction.write() = direction.normalize();
    }

    // Attenuation
    pub fn get_constant_attenuation(&self) -> f32 {
        *self.constant_attenuation.read()
    }

    pub fn set_constant_attenuation(&self, value: f32) {
        *self.constant_attenuation.write() = value;
    }

    pub fn get_linear_attenuation(&self) -> f32 {
        *self.linear_attenuation.read()
    }

    pub fn set_linear_attenuation(&self, value: f32) {
        *self.linear_attenuation.write() = value;
    }

    pub fn get_quadratic_attenuation(&self) -> f32 {
        *self.quadratic_attenuation.read()
    }

    pub fn set_quadratic_attenuation(&self, value: f32) {
        *self.quadratic_attenuation.write() = value;
    }

    // Spot properties
    pub fn get_spot_inner_angle(&self) -> f32 {
        *self.spot_inner_angle.read()
    }

    pub fn set_spot_inner_angle(&self, angle: f32) {
        *self.spot_inner_angle.write() = angle;
    }

    pub fn get_spot_outer_angle(&self) -> f32 {
        *self.spot_outer_angle.read()
    }

    pub fn set_spot_outer_angle(&self, angle: f32) {
        *self.spot_outer_angle.write() = angle;
    }

    // Shadow properties
    pub fn casts_shadows(&self) -> bool {
        *self.casts_shadows.read()
    }

    pub fn set_casts_shadows(&self, casts: bool) {
        *self.casts_shadows.write() = casts;
    }

    pub fn get_shadow_distance(&self) -> f32 {
        *self.shadow_distance.read()
    }

    pub fn set_shadow_distance(&self, distance: f32) {
        *self.shadow_distance.write() = distance;
    }

    pub fn get_shadow_intensity(&self) -> f32 {
        *self.shadow_intensity.read()
    }

    pub fn set_shadow_intensity(&self, intensity: f32) {
        *self.shadow_intensity.write() = intensity;
    }

    /// Set typical point light attenuation
    pub fn set_point_light_attenuation(&self, range: f32) {
        *self.constant_attenuation.write() = 1.0;
        *self.linear_attenuation.write() = 0.09;
        *self.quadratic_attenuation.write() = 0.032;
    }
}

/// Directional light
pub struct DirectionalLight {
    light: Light,
}

impl DirectionalLight {
    pub fn new(name: &str) -> Self {
        Self {
            light: Light::new(name, LightType::Directional),
        }
    }

    pub fn get_light(&self) -> &Light {
        &self.light
    }
}

/// Point light
pub struct PointLight {
    light: Light,
}

impl PointLight {
    pub fn new(name: &str) -> Self {
        Self {
            light: Light::new(name, LightType::Point),
        }
    }

    pub fn get_light(&self) -> &Light {
        &self.light
    }

    pub fn set_range(&self, range: f32) {
        self.light.set_point_light_attenuation(range);
    }
}

/// Spot light
pub struct SpotLight {
    light: Light,
}

impl SpotLight {
    pub fn new(name: &str) -> Self {
        let light = Light::new(name, LightType::Spot);
        Self { light }
    }

    pub fn get_light(&self) -> &Light {
        &self.light
    }

    pub fn set_inner_cone(&self, angle: f32) {
        self.light.set_spot_inner_angle(angle);
    }

    pub fn set_outer_cone(&self, angle: f32) {
        self.light.set_spot_outer_angle(angle);
    }
}
