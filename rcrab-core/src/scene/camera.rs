// Camera - View camera for rendering

use crate::math::{Mat4, Transform, Vec3, Vec4};
use parking_lot::RwLock;

/// Camera projection type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Projection {
    Perspective,
    Orthographic,
}

/// Camera frustum
pub struct Camera {
    // Transform
    location: RwLock<Vec3>,
    rotation: RwLock<Mat4>,
    view_matrix: RwLock<Mat4>,
    projection_matrix: RwLock<Mat4>,

    // Projection parameters
    projection: RwLock<Projection>,
    fov: RwLock<f32>,           // Field of view (radians)
    aspect: RwLock<f32>,        // Aspect ratio
    near: RwLock<f32>,          // Near plane
    far: RwLock<f32>,           // Far plane
    frustum_planes: RwLock<[Mat4; 6]>, // Frustum planes

    // Orthographic parameters
    left: RwLock<f32>,
    right: RwLock<f32>,
    top: RwLock<f32>,
    bottom: RwLock<f32>,

    // Window size for viewport
    width: RwLock<u32>,
    height: RwLock<u32>,

    // Culling
    culling_mode: RwLock<CullingMode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullingMode {
    Off,
    Front,
    Back,
    FrontAndBack,
}

impl Default for CullingMode {
    fn default() -> Self {
        Self::Back
    }
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cam = Self {
            location: RwLock::new(Vec3::new(0.0, 0.0, 10.0)),
            rotation: RwLock::new(Mat4::IDENTITY),
            view_matrix: RwLock::new(Mat4::IDENTITY),
            projection_matrix: RwLock::new(Mat4::IDENTITY),
            projection: RwLock::new(Projection::Perspective),
            fov: RwLock::new(std::f32::consts::PI / 3.0), // 60 degrees
            aspect: RwLock::new(width as f32 / height as f32),
            near: RwLock::new(0.1),
            far: RwLock::new(1000.0),
            frustum_planes: RwLock::new([Mat4::IDENTITY; 6]),
            left: RwLock::new(-1.0),
            right: RwLock::new(1.0),
            top: RwLock::new(1.0),
            bottom: RwLock::new(-1.0),
            width: RwLock::new(width),
            height: RwLock::new(height),
            culling_mode: RwLock::new(CullingMode::Back),
        };

        cam.update_projection();
        cam.update_view();

        cam
    }

    /// Get camera location
    pub fn get_location(&self) -> Vec3 {
        *self.location.read()
    }

    /// Set camera location
    pub fn set_location(&self, location: Vec3) {
        *self.location.write() = location;
        self.update_view();
    }

    /// Move camera by offset
    pub fn move_by(&self, offset: Vec3) {
        *self.location.write() += offset;
        self.update_view();
    }

    /// Get view matrix
    pub fn get_view_matrix(&self) -> Mat4 {
        *self.view_matrix.read()
    }

    /// Get projection matrix
    pub fn get_projection_matrix(&self) -> Mat4 {
        *self.projection_matrix.read()
    }

    /// Get combined view-projection matrix
    pub fn get_view_projection_matrix(&self) -> Mat4 {
        self.get_projection_matrix() * self.get_view_matrix()
    }

    /// Get field of view (radians)
    pub fn get_fov(&self) -> f32 {
        *self.fov.read()
    }

    /// Set field of view (radians)
    pub fn set_fov(&self, fov: f32) {
        *self.fov.write() = fov;
        self.update_projection();
    }

    /// Get aspect ratio
    pub fn get_aspect(&self) -> f32 {
        *self.aspect.read()
    }

    /// Set aspect ratio
    pub fn set_aspect(&self, aspect: f32) {
        *self.aspect.write() = aspect;
        self.update_projection();
    }

    /// Get near plane
    pub fn get_near(&self) -> f32 {
        *self.near.read()
    }

    /// Set near plane
    pub fn set_near(&self, near: f32) {
        *self.near.write() = near;
        self.update_projection();
    }

    /// Get far plane
    pub fn get_far(&self) -> f32 {
        *self.far.read()
    }

    /// Set far plane
    pub fn set_far(&self, far: f32) {
        *self.far.write() = far;
        self.update_projection();
    }

    /// Get width
    pub fn get_width(&self) -> u32 {
        *self.width.read()
    }

    /// Get height
    pub fn get_height(&self) -> u32 {
        *self.height.read()
    }

    /// Set viewport size
    pub fn set_viewport_size(&self, width: u32, height: u32) {
        *self.width.write() = width;
        *self.height.write() = height;
        *self.aspect.write() = width as f32 / height as f32;
        self.update_projection();
    }

    /// Look at a target
    pub fn look_at(&self, target: Vec3, up: Vec3) {
        let location = self.get_location();
        let direction = (target - location).normalize();
        let right = direction.cross(up).normalize();
        let new_up = right.cross(direction);

        *self.rotation.write() = Mat4::from_cols(
            right.extend(0.0),
            new_up.extend(0.0),
            (-direction).extend(0.0),
            Vec4::ZERO
        );
        self.update_view();
    }

    /// Get forward direction
    pub fn get_forward(&self) -> Vec3 {
        -self.rotation.read().z_axis.truncate()
    }

    /// Get up direction
    pub fn get_up(&self) -> Vec3 {
        self.rotation.read().y_axis.truncate()
    }

    /// Get right direction
    pub fn get_right(&self) -> Vec3 {
        self.rotation.read().x_axis.truncate()
    }

    /// Get culling mode
    pub fn get_culling_mode(&self) -> CullingMode {
        *self.culling_mode.read()
    }

    /// Set culling mode
    pub fn set_culling_mode(&self, mode: CullingMode) {
        *self.culling_mode.write() = mode;
    }

    /// Get projection type
    pub fn get_projection(&self) -> Projection {
        *self.projection.read()
    }

    /// Set projection type
    pub fn set_projection(&self, projection: Projection) {
        *self.projection.write() = projection;
        self.update_projection();
    }

    /// Set orthographic bounds
    pub fn set_orthographic(&self, left: f32, right: f32, bottom: f32, top: f32) {
        *self.left.write() = left;
        *self.right.write() = right;
        *self.bottom.write() = bottom;
        *self.top.write() = top;
        *self.projection.write() = Projection::Orthographic;
        self.update_projection();
    }

    /// Update view matrix
    fn update_view(&self) {
        let location = self.get_location();
        let rotation = *self.rotation.read();

        // Create view matrix (inverse of camera transform)
        let mut view = rotation;
        view.w_axis = Vec4::ZERO;
        let translation = Mat4::from_translation(-location);
        *self.view_matrix.write() = view * translation;
    }

    /// Update projection matrix
    fn update_projection(&self) {
        let proj = match *self.projection.read() {
            Projection::Perspective => {
                let fov = *self.fov.read();
                let aspect = *self.aspect.read();
                let near = *self.near.read();
                let far = *self.far.read();

                let f = 1.0 / (fov / 2.0).tan();
                let range_inv = 1.0 / (near - far);

                Mat4::from_cols(
                    Vec4::new(f / aspect, 0.0, 0.0, 0.0),
                    Vec4::new(0.0, f, 0.0, 0.0),
                    Vec4::new(0.0, 0.0, (near + far) * range_inv, -1.0),
                    Vec4::new(0.0, 0.0, near * far * range_inv * 2.0, 0.0),
                )
            }
            Projection::Orthographic => {
                let left = *self.left.read();
                let right = *self.right.read();
                let bottom = *self.bottom.read();
                let top = *self.top.read();
                let near = *self.near.read();
                let far = *self.far.read();

                Mat4::from_cols(
                    Vec4::new(2.0 / (right - left), 0.0, 0.0, 0.0),
                    Vec4::new(0.0, 2.0 / (top - bottom), 0.0, 0.0),
                    Vec4::new(0.0, 0.0, -2.0 / (far - near), 0.0),
                    Vec4::new(-(right + left) / (right - left), -(top + bottom) / (top - bottom), -(far + near) / (far - near), 1.0),
                )
            }
        };

        *self.projection_matrix.write() = proj;
    }
}
