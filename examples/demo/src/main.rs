// rCrabEngine Demo - FPS Crab Shooter (Low Budget Doom)
// Features: Ground terrain, WASD movement, mouse look, crab enemies, shooting

use bytemuck::{Pod, Zeroable};
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, error};
use wgpu::{
    util::DeviceExt,
    BlendState, BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
    Device, DeviceDescriptor, FragmentState, Instance, InstanceDescriptor, PipelineLayoutDescriptor,
    PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource,
    SurfaceConfiguration, TextureUsages, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState, VertexStepMode,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
};

const MOVE_SPEED: f32 = 8.0;
const JUMP_VELOCITY: f32 = 8.0;
const GRAVITY: f32 = -20.0;
const MOUSE_SENSITIVITY: f32 = 0.002;
const CRAB_SPEED: f32 = 2.5;
const PROJECTILE_SPEED: f32 = 40.0;
const SPAWN_DISTANCE: f32 = 25.0;

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    color: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3], uv: [f32; 2], color: [f32; 3]) -> Self {
        Self { position, normal, uv, color }
    }
}

fn create_box(size: [f32; 3], color: [f32; 3]) -> Vec<Vertex> {
    let (sx, sy, sz) = (size[0] / 2.0, size[1] / 2.0, size[2] / 2.0);
    let faces = [
        ([-sx, -sy, sz], [0.0, 0.0, 1.0]),  // Front
        ([sx, -sy, -sz], [0.0, 0.0, -1.0]), // Back
        ([-sx, sy, sz], [0.0, 1.0, 0.0]),    // Top
        ([-sx, -sy, -sz], [0.0, -1.0, 0.0]), // Bottom
        ([sx, -sy, sz], [1.0, 0.0, 0.0]),    // Right
        ([-sx, -sy, -sz], [-1.0, 0.0, 0.0]), // Left
    ];
    let uvs = [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0], [0.0, 1.0], [1.0, 0.0]];
    let mut vertices = Vec::new();
    for (pos, normal) in faces {
        for i in 0..6 {
            vertices.push(Vertex::new(pos, normal, uvs[i], color));
        }
    }
    vertices
}

fn create_sphere(radius: f32, segments: usize, color: [f32; 3]) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    for lat in 0..segments {
        let t1 = (lat as f32) * std::f32::consts::PI / segments as f32;
        let t2 = ((lat + 1) as f32) * std::f32::consts::PI / segments as f32;
        for lon in 0..segments {
            let p1 = (lon as f32) * 2.0 * std::f32::consts::PI / segments as f32;
            let p2 = ((lon + 1) as f32) * 2.0 * std::f32::consts::PI / segments as f32;

            let v = |theta: f32, phi: f32| -> Vertex {
                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.cos();
                let z = radius * theta.sin() * phi.sin();
                let n = [x / radius, y / radius, z / radius];
                let uv = [phi / (2.0 * std::f32::consts::PI), theta / std::f32::consts::PI];
                Vertex::new([x, y, z], n, uv, color)
            };

            vertices.push(v(t1, p1)); vertices.push(v(t1, p2)); vertices.push(v(t2, p2));
            vertices.push(v(t1, p1)); vertices.push(v(t2, p2)); vertices.push(v(t2, p1));
        }
    }
    vertices
}

fn create_ground(size: f32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    let grid = 40;
    let step = size / grid as f32;

    for z in 0..grid {
        for x in 0..grid {
            let wx = x as f32 * step - size / 2.0;
            let wz = z as f32 * step - size / 2.0;
            let wx2 = wx + step;
            let wz2 = wz + step;

            // Checkered grass pattern
            let c1 = if (x + z) % 2 == 0 { [0.15, 0.35, 0.1] } else { [0.12, 0.3, 0.08] };
            let c2 = if (x + 1 + z) % 2 == 0 { [0.15, 0.35, 0.1] } else { [0.12, 0.3, 0.08] };
            let c3 = if (x + z + 1) % 2 == 0 { [0.15, 0.35, 0.1] } else { [0.12, 0.3, 0.08] };
            let c4 = if (x + 1 + z + 1) % 2 == 0 { [0.15, 0.35, 0.1] } else { [0.12, 0.3, 0.08] };

            vertices.push(Vertex::new([wx, 0.0, wz], [0.0, 1.0, 0.0], [0.0, 0.0], c1));
            vertices.push(Vertex::new([wx2, 0.0, wz], [0.0, 1.0, 0.0], [1.0, 0.0], c2));
            vertices.push(Vertex::new([wx2, 0.0, wz2], [0.0, 1.0, 0.0], [1.0, 1.0], c4));

            vertices.push(Vertex::new([wx, 0.0, wz], [0.0, 1.0, 0.0], [0.0, 0.0], c1));
            vertices.push(Vertex::new([wx2, 0.0, wz2], [0.0, 1.0, 0.0], [1.0, 1.0], c4));
            vertices.push(Vertex::new([wx, 0.0, wz2], [0.0, 1.0, 0.0], [0.0, 1.0], c3));
        }
    }
    vertices
}

// ============ CAMERA ============

struct Camera {
    position: glam::Vec3,
    yaw: f32,
    pitch: f32,
    velocity: glam::Vec3,
    on_ground: bool,
}

impl Camera {
    fn new() -> Self {
        Self { position: glam::Vec3::new(0.0, 2.0, 15.0), yaw: 0.0, pitch: -0.1, velocity: glam::Vec3::ZERO, on_ground: false }
    }

    fn get_forward(&self) -> glam::Vec3 {
        glam::Vec3::new(self.yaw.cos() * self.pitch.cos(), self.pitch.sin(), self.yaw.sin() * self.pitch.cos())
    }

    fn get_right(&self) -> glam::Vec3 {
        glam::Vec3::new(self.yaw.cos(), 0.0, -self.yaw.sin()).normalize()
    }

    fn update(&mut self, dt: f32, keys: &HashMap<String, bool>) {
        let forward = self.get_forward();
        let right = self.get_right();
        let mut move_dir = glam::Vec3::ZERO;

        if keys.get("w").copied().unwrap_or(false) { move_dir += forward; }
        if keys.get("s").copied().unwrap_or(false) { move_dir -= forward; }
        if keys.get("a").copied().unwrap_or(false) { move_dir -= right; }
        if keys.get("d").copied().unwrap_or(false) { move_dir += right; }

        if move_dir.length() > 0.0 { move_dir = move_dir.normalize() * MOVE_SPEED; }

        // Apply gravity
        self.velocity.y += GRAVITY * dt;
        self.velocity.x = move_dir.x;
        self.velocity.z = move_dir.z;

        self.position += self.velocity * dt;

        // Ground collision
        if self.position.y < 2.0 {
            self.position.y = 2.0;
            self.velocity.y = 0.0;
            self.on_ground = true;
        } else {
            self.on_ground = false;
        }
    }

    fn jump(&mut self) {
        if self.on_ground { self.velocity.y = JUMP_VELOCITY; self.on_ground = false; }
    }

    fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * MOUSE_SENSITIVITY;
        self.pitch -= dy * MOUSE_SENSITIVITY;
        // Clamp pitch to avoid flipping
        self.pitch = self.pitch.clamp(-1.5, 1.5);
    }
}

// ============ SCENE OBJECTS ============

struct SceneObject {
    _position: glam::Vec3,
    rotation: f32,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
}

impl SceneObject {
    fn new(device: &Device, position: glam::Vec3, vertices: Vec<Vertex>) -> Self {
        let vertex_count = vertices.len();
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("VB"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });
        Self { _position: position, rotation: 0.0, vertex_buffer: vb, vertex_count }
    }

    fn update(&mut self, _time: f32) {
        self.rotation += 0.01;
    }
}

// ============ CRAB ENEMY ============

struct Crab {
    position: glam::Vec3,
    alive: bool,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
}

impl Crab {
    fn new(device: &Device, position: glam::Vec3) -> Self {
        // Create a simple crab shape (flattened sphere with legs)
        let mut vertices = Vec::new();
        let body_color = [0.8, 0.3, 0.2]; // Reddish crab color
        let leg_color = [0.6, 0.2, 0.15];

        // Body (flattened sphere)
        for lat in 0..8 {
            let t1 = (lat as f32) * std::f32::consts::PI / 8.0;
            let t2 = ((lat + 1) as f32) * std::f32::consts::PI / 8.0;
            for lon in 0..8 {
                let p1 = (lon as f32) * 2.0 * std::f32::consts::PI / 8.0;
                let p2 = ((lon + 1) as f32) * 2.0 * std::f32::consts::PI / 8.0;

                let v = |theta: f32, phi: f32| -> Vertex {
                    let x = 0.8 * theta.sin() * phi.cos();
                    let y = 0.3 * theta.cos(); // Flattened
                    let z = 0.8 * theta.sin() * phi.sin();
                    let n = [x / 0.8, y / 0.3, z / 0.8];
                    Vertex::new([x, y, z], n, [0.0, 0.0], body_color)
                };

                vertices.push(v(t1, p1)); vertices.push(v(t1, p2)); vertices.push(v(t2, p2));
                vertices.push(v(t1, p1)); vertices.push(v(t2, p2)); vertices.push(v(t2, p1));
            }
        }

        // Legs (small boxes sticking out)
        let leg_offsets = [
            (-0.7, 0.0, 0.3), (-0.7, 0.0, -0.3),
            (0.7, 0.0, 0.3), (0.7, 0.0, -0.3),
            (-0.5, 0.1, 0.5), (-0.5, 0.1, -0.5),
            (0.5, 0.1, 0.5), (0.5, 0.1, -0.5),
        ];
        for (lx, ly, lz) in leg_offsets {
            let leg = create_box([0.3, 0.1, 0.15], leg_color);
            for mut v in leg {
                v.position = [v.position[0] + lx, v.position[1] + ly, v.position[2] + lz];
                vertices.push(v);
            }
        }

        let vertex_count = vertices.len();
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Crab VB"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        Self { position, alive: true, vertex_buffer: vb, vertex_count }
    }

    fn update(&mut self, player_pos: glam::Vec3, dt: f32) {
        if !self.alive { return; }

        // Move toward player
        let direction = (player_pos - self.position).normalize();
        self.position += direction * CRAB_SPEED * dt;

        // Keep on ground
        if self.position.y < 0.5 {
            self.position.y = 0.5;
        }
    }

    fn get_position(&self) -> glam::Vec3 { self.position }
    fn set_dead(&mut self) { self.alive = false; }
    fn is_alive(&self) -> bool { self.alive }
}

// ============ PROJECTILE ============

struct Projectile {
    position: glam::Vec3,
    velocity: glam::Vec3,
    alive: bool,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
}

impl Projectile {
    fn new(device: &Device, position: glam::Vec3, direction: glam::Vec3) -> Self {
        let velocity = direction.normalize() * PROJECTILE_SPEED;
        let vertices = create_sphere(0.15, 6, [1.0, 1.0, 0.2]); // Yellow bullet
        let vertex_count = vertices.len();
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Projectile VB"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
        });

        Self { position, velocity, alive: true, vertex_buffer: vb, vertex_count }
    }

    fn update(&mut self, dt: f32) {
        if !self.alive { return; }
        self.position += self.velocity * dt;
        // Kill if too far
        if self.position.length() > 50.0 {
            self.alive = false;
        }
    }

    fn get_position(&self) -> glam::Vec3 { self.position }
    fn set_dead(&mut self) { self.alive = false; }
    fn is_alive(&self) -> bool { self.alive }
}

// ============ MAIN APP ============

struct DemoApp {
    surface: wgpu::Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    model_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    ground_vb: wgpu::Buffer,
    ground_count: usize,
    objects: Vec<SceneObject>,
    crabs: Vec<Crab>,
    projectiles: Vec<Projectile>,
    camera: Camera,
    keys: HashMap<String, bool>,
    last_time: Instant,
    score: u32,
    spawn_timer: f32,
}

// Camera uniforms: view matrix (mat4) + projection matrix (mat4) = 32 floats
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct ModelUniforms {
    model: [[f32; 4]; 4],
}

impl DemoApp {
    async fn new(window: winit::window::Window) -> Self {
        let window_ptr = Box::new(window);
        let window_ref: &'static winit::window::Window = Box::leak(window_ptr);

        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window_ref).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptionsBase::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&DeviceDescriptor {
            label: Some("Device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
            experimental_features: Default::default(),
        }).await.unwrap();

        let format = surface.get_capabilities(&adapter).formats[0];
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format,
            width: window_ref.inner_size().width,
            height: window_ref.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        // Shader with lighting and camera matrices + model transform
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(r#"
                struct CameraUniforms {
                    view: mat4x4<f32>,
                    projection: mat4x4<f32>,
                }
                struct ModelUniforms {
                    model: mat4x4<f32>,
                }
                @group(0) @binding(0) var<uniform> camera: CameraUniforms;
                @group(0) @binding(1) var<uniform> model: ModelUniforms;

                struct VertexInput {
                    @location(0) position: vec3<f32>,
                    @location(1) normal: vec3<f32>,
                    @location(2) uv: vec2<f32>,
                    @location(3) color: vec3<f32>,
                }

                struct VertexOutput {
                    @builtin(position) position: vec4<f32>,
                    @location(0) normal: vec3<f32>,
                    @location(1) color: vec3<f32>,
                }

                @vertex
                fn vs_main(input: VertexInput) -> VertexOutput {
                    var output: VertexOutput;
                    let world_pos = model.model * vec4<f32>(input.position, 1.0);
                    output.position = camera.projection * camera.view * world_pos;
                    output.normal = input.normal;
                    output.color = input.color;
                    return output;
                }

                @fragment
                fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                    let n = normalize(input.normal);
                    // Sun light
                    let sun_dir = normalize(vec3<f32>(0.6, 0.8, 0.4));
                    let diff = max(dot(n, sun_dir), 0.0);
                    // Ambient
                    let ambient = vec3<f32>(0.25, 0.3, 0.35);
                    let sun = vec3<f32>(1.0, 0.95, 0.8) * diff * 0.7;
                    return vec4<f32>(input.color * (ambient + sun), 1.0);
                }
            "#.into()),
        });

        // Create uniform buffer for camera matrices
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<CameraUniforms>() as wgpu::BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create model uniform buffer (for per-object positioning)
        let model_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Model Buffer"),
            size: std::mem::size_of::<ModelUniforms>() as wgpu::BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create bind group layout with 2 bindings
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("PL"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        // Create bind group with 2 entries
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: model_buffer.as_entire_binding(),
                },
            ],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("RP"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute { format: VertexFormat::Float32x3, offset: 0, shader_location: 0 },
                        VertexAttribute { format: VertexFormat::Float32x3, offset: 12, shader_location: 1 },
                        VertexAttribute { format: VertexFormat::Float32x2, offset: 24, shader_location: 2 },
                        VertexAttribute { format: VertexFormat::Float32x3, offset: 32, shader_location: 3 },
                    ],
                }],
            },
            primitive: PrimitiveState { topology: PrimitiveTopology::TriangleList, ..Default::default() },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState { format, blend: Some(BlendState::ALPHA_BLENDING), write_mask: ColorWrites::ALL })],
            }),
            depth_stencil: None, multisample: Default::default(), multiview_mask: Default::default(), cache: None,
        });

        // Create ground
        let ground = create_ground(80.0);
        let ground_count = ground.len();
        let ground_vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Ground"),
            contents: bytemuck::cast_slice(&ground),
            usage: BufferUsages::VERTEX,
        });

        // Create objects scattered around
        let mut objects = Vec::new();

        // Colored boxes
        let box_data = [
            ([1.0, 0.2, 0.2], glam::Vec3::new(-8.0, 0.5, -8.0)),
            ([0.2, 1.0, 0.2], glam::Vec3::new(6.0, 0.5, -10.0)),
            ([0.2, 0.4, 1.0], glam::Vec3::new(-12.0, 0.5, 5.0)),
            ([1.0, 1.0, 0.2], glam::Vec3::new(10.0, 0.5, 8.0)),
            ([1.0, 0.5, 0.0], glam::Vec3::new(0.0, 0.5, -15.0)),
            ([0.5, 0.0, 1.0], glam::Vec3::new(-5.0, 0.5, 12.0)),
            ([0.0, 1.0, 1.0], glam::Vec3::new(15.0, 0.5, -5.0)),
            ([1.0, 0.0, 0.5], glam::Vec3::new(-15.0, 0.5, -15.0)),
        ];
        for (color, pos) in box_data {
            objects.push(SceneObject::new(&device, pos, create_box([1.0, 1.0, 1.0], color)));
        }

        // Floating spheres (animated)
        let sphere_positions = [
            (glam::Vec3::new(4.0, 2.5, 2.0), [0.9, 0.6, 0.2]),
            (glam::Vec3::new(-3.0, 3.5, -4.0), [0.2, 0.6, 0.9]),
            (glam::Vec3::new(0.0, 4.5, -8.0), [0.9, 0.2, 0.6]),
            (glam::Vec3::new(7.0, 2.0, -3.0), [0.3, 0.9, 0.3]),
            (glam::Vec3::new(-6.0, 3.0, 6.0), [0.9, 0.9, 0.2]),
            (glam::Vec3::new(12.0, 2.5, 0.0), [0.6, 0.2, 0.9]),
        ];
        for (pos, color) in sphere_positions {
            objects.push(SceneObject::new(&device, pos, create_sphere(0.6, 12, color)));
        }

        // Tall tower of boxes
        for i in 0..8 {
            let y = 0.5 + i as f32 * 1.05;
            let color = if i % 3 == 0 { [0.8, 0.2, 0.2] } else if i % 3 == 1 { [0.2, 0.2, 0.8] } else { [0.2, 0.8, 0.2] };
            objects.push(SceneObject::new(&device, glam::Vec3::new(18.0, y, 0.0), create_box([0.8, 0.8, 0.8], color)));
        }

        // Another tower
        for i in 0..6 {
            let y = 0.5 + i as f32 * 1.05;
            let color = if i % 2 == 0 { [1.0, 0.8, 0.2] } else { [0.2, 0.8, 1.0] };
            objects.push(SceneObject::new(&device, glam::Vec3::new(-18.0, y, 10.0), create_box([0.9, 0.9, 0.9], color)));
        }

        // Spawn initial crabs
        let mut crabs = Vec::new();
        let crab_positions = [
            glam::Vec3::new(10.0, 0.5, -5.0),
            glam::Vec3::new(-8.0, 0.5, 10.0),
            glam::Vec3::new(5.0, 0.5, 15.0),
            glam::Vec3::new(-15.0, 0.5, -10.0),
            glam::Vec3::new(0.0, 0.5, -20.0),
        ];
        for pos in crab_positions {
            crabs.push(Crab::new(&device, pos));
        }

        let projectiles = Vec::new();

        Self { surface, device, queue, config, pipeline, uniform_buffer, model_buffer, bind_group, ground_vb, ground_count, objects, crabs, projectiles, camera: Camera::new(), keys: HashMap::new(), last_time: Instant::now(), score: 0, spawn_timer: 0.0 }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn handle_key(&mut self, pressed: bool, key: &str) {
        if key == " " && pressed { self.camera.jump(); }
        else if pressed { self.keys.insert(key.to_string(), true); }
        else { self.keys.remove(key); }
    }

    fn handle_mouse_motion(&mut self, dx: f64, dy: f64) {
        self.camera.rotate(dx as f32, dy as f32);
    }

    fn shoot(&mut self) {
        // Spawn projectile from camera position
        let pos = self.camera.position;
        let dir = self.camera.get_forward();
        self.projectiles.push(Projectile::new(&self.device, pos, dir));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let dt = self.last_time.elapsed().as_secs_f32();
        self.last_time = Instant::now();

        self.camera.update(dt, &self.keys);

        let time = Instant::now().elapsed().as_secs_f32();

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor::default());

        // Update and animate objects
        for obj in &mut self.objects {
            obj.update(time);
        }

        // Update crabs - move toward player
        let player_pos = self.camera.position;
        for crab in &mut self.crabs {
            crab.update(player_pos, dt);
        }

        // Update projectiles
        for proj in &mut self.projectiles {
            proj.update(dt);
        }

        // Collision detection: projectile vs crab
        for proj in &mut self.projectiles {
            if !proj.is_alive() { continue; }
            let proj_pos = proj.get_position();
            for crab in &mut self.crabs {
                if !crab.is_alive() { continue; }
                let crab_pos = crab.get_position();
                if (proj_pos - crab_pos).length() < 1.2 {
                    crab.set_dead();
                    proj.set_dead();
                    self.score += 100;
                    info!("Crab killed! Score: {}", self.score);
                }
            }
        }

        // Remove dead entities
        self.projectiles.retain(|p| p.is_alive());
        self.crabs.retain(|c| c.is_alive());

        // Spawn new crabs periodically
        self.spawn_timer += dt;
        if self.spawn_timer > 3.0 && self.crabs.len() < 10 {
            self.spawn_timer = 0.0;
            // Spawn crab at random position around player
            let angle = (self.spawn_timer * 7.0).sin() * std::f32::consts::PI * 2.0;
            let dist = SPAWN_DISTANCE;
            let spawn_pos = player_pos + glam::Vec3::new(angle.cos() * dist, 0.5, angle.sin() * dist);
            self.crabs.push(Crab::new(&self.device, spawn_pos));
            info!("New crab spawned! Total crabs: {}", self.crabs.len());
        }

        // Camera matrices
        let cam = &self.camera;
        let view_mat = glam::Mat4::look_at_rh(cam.position, cam.position + cam.get_forward(), glam::Vec3::Y);
        let proj_mat = glam::Mat4::perspective_rh(60.0f32.to_radians(), self.config.width as f32 / self.config.height as f32, 0.1, 1000.0);

        // Update uniform buffer
        let uniforms = CameraUniforms {
            view: view_mat.to_cols_array_2d(),
            projection: proj_mat.to_cols_array_2d(),
        };
        self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&uniforms));

        {
            let mut rp = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view, resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.45, g: 0.55, b: 0.7, a: 1.0 }), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None, multiview_mask: Default::default(),
            });

            rp.set_pipeline(&self.pipeline);
            rp.set_bind_group(0, &self.bind_group, &[]);

            // Set identity model matrix for ground and static objects
            let identity = ModelUniforms { model: glam::Mat4::IDENTITY.to_cols_array_2d() };
            self.queue.write_buffer(&self.model_buffer, 0, bytemuck::bytes_of(&identity));

            // Draw ground
            rp.set_vertex_buffer(0, self.ground_vb.slice(..));
            rp.draw(0..self.ground_count as u32, 0..1);

            // Draw objects
            for obj in &self.objects {
                rp.set_vertex_buffer(0, obj.vertex_buffer.slice(..));
                rp.draw(0..obj.vertex_count as u32, 0..1);
            }

            // Draw crabs
            for crab in &self.crabs {
                // Create model matrix from crab position
                let pos = crab.get_position();
                let model_mat = glam::Mat4::from_translation(pos);
                let model_uniforms = ModelUniforms { model: model_mat.to_cols_array_2d() };
                self.queue.write_buffer(&self.model_buffer, 0, bytemuck::bytes_of(&model_uniforms));

                rp.set_vertex_buffer(0, crab.vertex_buffer.slice(..));
                rp.draw(0..crab.vertex_count as u32, 0..1);
            }

            // Draw projectiles
            for proj in &self.projectiles {
                let pos = proj.get_position();
                let model_mat = glam::Mat4::from_translation(pos);
                let model_uniforms = ModelUniforms { model: model_mat.to_cols_array_2d() };
                self.queue.write_buffer(&self.model_buffer, 0, bytemuck::bytes_of(&model_uniforms));

                rp.set_vertex_buffer(0, proj.vertex_buffer.slice(..));
                rp.draw(0..proj.vertex_count as u32, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    info!("rCrabEngine FPS - WASD move, MOUSE look, CLICK shoot, SPACE jump");

    let event_loop = EventLoop::new().unwrap();

    #[allow(deprecated)]
    let window = event_loop.create_window(winit::window::WindowAttributes::default()
        .with_title("rCrabEngine - Crab Shooter (FPS)")
        .with_inner_size(winit::dpi::LogicalSize::new(1280, 720)))
        .unwrap();

    // Hide cursor for FPS controls
    window.set_cursor_visible(false);

    let window_id = window.id();
    let mut app = pollster::block_on(DemoApp::new(window));

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, _target| {
        match event {
            Event::WindowEvent { event, window_id: id } if id == window_id => {
                match event {
                    WindowEvent::CloseRequested => std::process::exit(0),
                    WindowEvent::Resized(sz) => app.resize(sz.width, sz.height),
                    WindowEvent::KeyboardInput { event, .. } => {
                        let pressed = event.state == winit::event::ElementState::Pressed;
                        if let Some(key) = event.logical_key.to_text() {
                            let key = key.to_lowercase();
                            if key == "w" || key == "a" || key == "s" || key == "d" || key == " " {
                                app.handle_key(pressed, &key);
                            }
                        }
                    }
                    WindowEvent::MouseInput { button, state, .. } => {
                        // Left click to shoot
                        if button == winit::event::MouseButton::Left && state == winit::event::ElementState::Pressed {
                            app.shoot();
                        }
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent { event, .. } => {
                // Handle mouse motion from DeviceEvent
                if let winit::event::DeviceEvent::MouseMotion { delta } = event {
                    app.handle_mouse_motion(delta.0, delta.1);
                }
            }
            Event::AboutToWait => {
                if let Err(e) = app.render() { error!("Render: {:?}", e); }
            }
            _ => {}
        }
    });
}
