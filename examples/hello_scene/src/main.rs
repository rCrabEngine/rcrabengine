// Hello Scene - Scene graph with multiple geometries and pivot rotation
// Port of jME3 HelloNode example

use bytemuck::{Pod, Zeroable};
use tracing::{info, error};
use wgpu::{
    util::DeviceExt,
    BlendComponent, BlendState, BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
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

#[derive(Pod, Zeroable, Clone, Copy)]
#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }
}

// Cube at position with color
struct Cube {
    position: [f32; 3],
    color: [f32; 3],
    size: f32,
}

struct HelloScene {
    window: &'static winit::window::Window,
    surface: wgpu::Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
    cubes: Vec<Cube>,
    pivot_rotation: f32,
}

impl HelloScene {
    async fn new(window: winit::window::Window) -> Self {
        // Leak the window to get 'static lifetime for the surface
        let window_ptr = Box::new(window);
        let window_ref: &'static winit::window::Window = Box::leak(window_ptr);

        let instance = Instance::new(&InstanceDescriptor::default());
        let surface = instance.create_surface(window_ref).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase::default())
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: Some("Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: Default::default(),
                    trace: Default::default(),
                    experimental_features: Default::default(),
                },
            )
            .await
            .unwrap();

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

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(
                r#"
                struct VertexInput {
                    @location(0) position: vec3<f32>,
                    @location(1) color: vec3<f32>,
                }

                struct VertexOutput {
                    @builtin(position) position: vec4<f32>,
                    @location(0) color: vec3<f32>,
                }

                @vertex
                fn vs_main(input: VertexInput) -> VertexOutput {
                    var output: VertexOutput;
                    // Simple perspective projection
                    let aspect = 800.0 / 600.0;
                    let fov = 1.0;
                    let z = 5.0;
                    let x = input.position.x / (input.position.z + z) / aspect * fov;
                    let y = input.position.y / (input.position.z + z) * fov;
                    output.position = vec4<f32>(x, y, (input.position.z + z) / 20.0, 1.0);
                    output.color = input.color;
                    return output;
                }

                @fragment
                fn fs_main(@location(0) color: vec3<f32>) -> @location(0) vec4<f32> {
                    return vec4<f32>(color, 1.0);
                }
                "#
                .into(),
            ),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: VertexStepMode::Vertex,
                    attributes: &[
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        VertexAttribute {
                            format: VertexFormat::Float32x3,
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format,
                    blend: Some(BlendState {
                        color: BlendComponent::REPLACE,
                        alpha: BlendComponent::REPLACE,
                    }),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: Default::default(),
            cache: None,
        });

        // Define cubes - blue at (1, -1, 1), red at (1, 3, 1)
        let cubes = vec![
            Cube { position: [1.0, -1.0, 1.0], color: [0.0, 0.0, 1.0], size: 1.0 },  // Blue
            Cube { position: [1.0, 3.0, 1.0], color: [1.0, 0.0, 0.0], size: 1.0 },   // Red
        ];

        // Create cube geometry (just 1 cube, we'll translate in shader)
        let vertices = create_cube();
        let vertex_count = vertices.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self {
            window: window_ref,
            surface,
            device,
            queue,
            config,
            pipeline,
            vertex_buffer,
            vertex_count,
            cubes,
            pivot_rotation: 0.0,
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        // Update pivot rotation
        self.pivot_rotation += 0.02;
        let pivot_sin = self.pivot_rotation.sin();
        let pivot_cos = self.pivot_rotation.cos();

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.1,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: Default::default(),
            });

            render_pass.set_pipeline(&self.pipeline);

            // Draw each cube with transformed position (pivot rotation)
            for cube in &self.cubes {
                // Apply pivot rotation around origin
                let rotated_x = cube.position[0] * pivot_cos - cube.position[2] * pivot_sin;
                let rotated_z = cube.position[0] * pivot_sin + cube.position[2] * pivot_cos;

                // Create transformed vertices on CPU
                let transformed = self.create_transformed_cube(rotated_x, cube.position[1], rotated_z, cube.size, cube.color);
                self.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&transformed));

                render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
                render_pass.draw(0..self.vertex_count as u32, 0..1);
            }
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }

    fn create_transformed_cube(&self, tx: f32, ty: f32, tz: f32, size: f32, color: [f32; 3]) -> Vec<Vertex> {
        let s = size / 2.0;
        let vertices = create_cube();
        vertices.iter().map(|v| {
            Vertex::new(
                [v.position[0] * size + tx, v.position[1] * size + ty, v.position[2] * size + tz],
                color,
            )
        }).collect()
    }
}

fn create_cube() -> Vec<Vertex> {
    let face_front = [
        Vertex::new([-0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
    ];
    let face_back = [
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
    ];
    let face_top = [
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
    ];
    let face_bottom = [
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
    ];
    let face_right = [
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
    ];
    let face_left = [
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, -0.5, -0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, 0.5], [1.0, 1.0, 1.0]),
        Vertex::new([-0.5, 0.5, -0.5], [1.0, 1.0, 1.0]),
    ];

    let mut vertices = Vec::new();
    vertices.extend(face_front);
    vertices.extend(face_back);
    vertices.extend(face_top);
    vertices.extend(face_bottom);
    vertices.extend(face_right);
    vertices.extend(face_left);
    vertices
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting rCrabEngine Hello Scene Example (jME3 HelloNode port)");

    let event_loop = EventLoop::new().unwrap();

    #[allow(deprecated)]
    let window = event_loop
        .create_window(winit::window::WindowAttributes::default()
            .with_title("rCrabEngine - Hello Scene")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600)))
        .unwrap();

    let window_id = window.id();
    let mut app = pollster::block_on(HelloScene::new(window));

    info!("Scene: Blue box at (1,-1,1), Red box at (1,3,1) - both rotating around pivot");

    #[allow(deprecated)]
    let _ = event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: id,
            } if id == window_id => {
                window_target.exit();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id: id,
            } if id == window_id => {
                app.resize(size.width, size.height);
            }
            Event::AboutToWait => {
                if let Err(e) = app.render() {
                    error!("Render error: {:?}", e);
                }
            }
            _ => {}
        }
    });
}
