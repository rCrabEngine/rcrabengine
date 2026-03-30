// Render manager and context

use crate::{Error, Result};
use parking_lot::RwLock;
use rcrab_core::{math::Vec2, scene::{Camera, Geometry, Light, Node}};
use std::sync::Arc;
use wgpu::{Device, Queue, Surface, SurfaceConfiguration, TextureFormat, PresentMode, Instance, InstanceDescriptor};

/// Main renderer struct
pub struct RenderManager {
    device: RwLock<Option<Device>>,
    queue: RwLock<Option<Queue>>,
    surface: RwLock<Option<Surface>>,
    config: RwLock<Option<SurfaceConfiguration>>,
    width: RwLock<u32>,
    height: RwLock<u32>,
    pipelines: RwLock<Vec<Arc<dyn RenderPipeline>>>,
    default_pipeline: RwLock<Option<Arc<dyn RenderPipeline>>>,
}

impl RenderManager {
    pub fn new() -> Self {
        Self {
            device: RwLock::new(None),
            queue: RwLock::new(None),
            surface: RwLock::new(None),
            config: RwLock::new(None),
            width: RwLock::new(1280),
            height: RwLock::new(720),
            pipelines: RwLock::new(Vec::new()),
            default_pipeline: RwLock::new(None),
        }
    }

    /// Initialize with a window (creates surface internally)
    pub fn initialize(&self, window: &wgpu::window::Window) -> Result<()> {
        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window)
            .map_err(|e| Error::Init(format!("Failed to create surface: {}", e)))?;

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase::default()))
            .ok_or_else(|| Error::Init("Failed to get GPU adapter".into()))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
                experimental_features: Default::default(),
            },
        )).map_err(|e| Error::Init(format!("Failed to create device: {}", e)))?;

        *self.device.write() = Some(device);
        *self.queue.write() = Some(queue);
        *self.surface.write() = Some(surface);

        // Configure surface
        let format = adapter.get_swap_chain_preferred_format(&surface);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            present_mode: PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            width: *self.width.read(),
            height: *self.height.read(),
        };

        if let Some(surf) = self.surface.read().as_ref() {
            surf.configure(&device, &config);
        }

        *self.config.write() = Some(config);

        Ok(())
    }

    /// Initialize with an existing surface
    pub fn initialize_with_surface(&self, surface: Surface) -> Result<()> {
        let instance = Instance::new(&InstanceDescriptor::default());

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase::default()))
            .ok_or_else(|| Error::Init("Failed to get GPU adapter".into()))?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Device"),
                required_features: wgpu::Features::default(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
                experimental_features: Default::default(),
            },
        )).map_err(|e| Error::Init(format!("Failed to create device: {}", e)))?;

        *self.device.write() = Some(device);
        *self.queue.write() = Some(queue);
        *self.surface.write() = Some(surface);

        // Configure surface
        let format = adapter.get_swap_chain_preferred_format(&surface);
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            present_mode: PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            width: *self.width.read(),
            height: *self.height.read(),
        };

        if let Some(surf) = self.surface.read().as_ref() {
            surf.configure(&device, &config);
        }

        *self.config.write() = Some(config);

        Ok(())
    }

    /// Resize the renderer
    pub fn resize(&self, width: u32, height: u32) {
        *self.width.write() = width;

        if let Some(config) = self.config.write().as_mut() {
            config.width = width;
            config.height = height;

            if let (Some(surface), Some(device)) =
                (self.surface.read().as_ref(), self.device.read().as_ref()) {
                surface.configure(device, config);
            }
        }
    }

    /// Get the device
    pub fn get_device(&self) -> Option<Device> {
        self.device.read().clone()
    }

    /// Get the queue
    pub fn get_queue(&self) -> Option<Queue> {
        self.queue.read().clone()
    }

    /// Get width
    pub fn get_width(&self) -> u32 {
        *self.width.read()
    }

    /// Get height
    pub fn get_height(&self) -> u32 {
        *self.height.read()
    }

    /// Render a frame
    pub fn render(&self, scene: &RenderContext) -> Result<()> {
        let device = self.device.read();
        let queue = self.queue.read();
        let surface = self.surface.read();

        if let (Some(device), Some(queue), Some(surface)) =
            (device.as_ref(), queue.as_ref(), surface.as_ref()) {

            let output = surface.get_current_texture()
                .map_err(|e| Error::Gpu(format!("Failed to get frame: {}", e)))?;

            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

            // Render scene
            self.render_scene(&mut encoder, &view, scene);

            queue.submit(Some(encoder.finish()));
            output.present();
        }

        Ok(())
    }

    fn render_scene(&self, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, scene: &RenderContext) {
        // Placeholder - will be implemented with proper render passes
    }

    /// Add a render pipeline
    pub fn add_pipeline(&self, pipeline: Arc<dyn RenderPipeline>) {
        self.pipelines.write().push(pipeline);
    }

    /// Set default pipeline
    pub fn set_default_pipeline(&self, pipeline: Arc<dyn RenderPipeline>) {
        *self.default_pipeline.write() = Some(pipeline);
    }
}

impl Default for RenderManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Render context containing scene data to render
pub struct RenderContext {
    pub scene: Arc<Node>,
    pub camera: Arc<Camera>,
    pub lights: Vec<Arc<Light>>,
    pub geometries: Vec<Arc<Geometry>>,
}

impl RenderContext {
    pub fn new(scene: Arc<Node>, camera: Arc<Camera>) -> Self {
        Self {
            scene,
            camera,
            lights: Vec::new(),
            geometries: Vec::new(),
        }
    }

    /// Collect all renderable geometries from the scene
    pub fn collect_geometries(&mut self) {
        self.geometries.clear();
        self.collect_from_node(&self.scene);
    }

    fn collect_from_node(&mut self, node: &Node) {
        for child in node.get_children_slice() {
            if let Some(geometry) = child.as_geometry() {
                if geometry.is_renderable() {
                    self.geometries.push(geometry);
                }
            }
        }
    }

    /// Collect all lights from the scene
    pub fn collect_lights(&mut self) {
        self.lights.clear();
        self.collect_lights_from_node(&self.scene);
    }

    fn collect_lights_from_node(&mut self, node: &Node) {
        // This would need Light attachment to nodes - simplified for now
    }
}

/// Extension trait for Spatial to get geometry
pub trait AsGeometry {
    fn as_geometry(&self) -> Option<Arc<Geometry>>;
}

impl AsGeometry for Arc<rcrab_core::scene::Spatial> {
    fn as_geometry(&self) -> Option<Arc<Geometry>> {
        // Would need proper downcasting - placeholder
        None
    }
}

/// Viewport for rendering
pub struct ViewPort {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub camera: Option<Arc<Camera>>,
    pub scene: Option<Arc<Node>>,
    pub clear_color: (f32, f32, f32, f32),
}

impl ViewPort {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            camera: None,
            scene: None,
            clear_color: (0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn with_camera(mut self, camera: Arc<Camera>) -> Self {
        self.camera = Some(camera);
        self
    }

    pub fn with_scene(mut self, scene: Arc<Node>) -> Self {
        self.scene = Some(scene);
        self
    }

    pub fn with_clear_color(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.clear_color = (r, g, b, a);
        self
    }
}

/// Render pipeline trait
pub trait RenderPipeline: Send + Sync {
    /// Get pipeline name
    fn get_name(&self) -> &str;

    /// Render a frame
    fn render(&self, context: &RenderContext, encoder: &mut wgpu::CommandEncoder, view: &wgpu::TextureView);
}
