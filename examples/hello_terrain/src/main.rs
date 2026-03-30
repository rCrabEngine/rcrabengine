// Hello Terrain - Procedural terrain with heightmap
// Port of jME3 HelloTerrain example

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

// Simple noise function for terrain
fn noise(x: f32, y: f32) -> f32 {
    let n = (x * 12.9898 + y * 78.233).sin() * 43758.5453;
    n - n.floor()
}

// Fractal noise for more natural terrain
fn fbm(x: f32, y: f32, octaves: usize) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut max_value = 0.0;

    for _ in 0..octaves {
        value += amplitude * noise(x * frequency, y * frequency);
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    value / max_value
}

fn create_terrain(size: usize, scale: f32) -> (Vec<Vertex>, usize) {
    let mut vertices = Vec::new();
    let step = scale / size as f32;
    let height_scale = 2.0;

    for z in 0..size {
        for x in 0..size {
            let wx = x as f32 * step - scale / 2.0;
            let wz = z as f32 * step - scale / 2.0;

            // Get height from noise
            let h = fbm(wx * 0.3, wz * 0.3, 4) * height_scale;
            let h_right = fbm((x as f32 + 1.0) * step * 0.3, wz * 0.3, 4) * height_scale;
            let h_down = fbm(wx * 0.3, (z as f32 + 1.0) * step * 0.3, 4) * height_scale;

            // Color based on height (green for grass, brown for dirt, white for snow)
            let color = if h > height_scale * 0.7 {
                [0.9, 0.95, 1.0] // Snow
            } else if h > height_scale * 0.4 {
                [0.4, 0.35, 0.25] // Dirt
            } else {
                [0.2, 0.5, 0.15] // Grass
            };

            let color_right = if h_right > height_scale * 0.7 {
                [0.9, 0.95, 1.0]
            } else if h_right > height_scale * 0.4 {
                [0.4, 0.35, 0.25]
            } else {
                [0.2, 0.5, 0.15]
            };

            let color_down = if h_down > height_scale * 0.7 {
                [0.9, 0.95, 1.0]
            } else if h_down > height_scale * 0.4 {
                [0.4, 0.35, 0.25]
            } else {
                [0.2, 0.5, 0.15]
            };

            // Create two triangles for the quad
            let x2 = (x + 1) as f32 * step - scale / 2.0;
            let z2 = (z + 1) as f32 * step - scale / 2.0;

            // Triangle 1
            vertices.push(Vertex::new([wx, h, wz], color));
            vertices.push(Vertex::new([x2, h_right, wz], color_right));
            vertices.push(Vertex::new([x2, h_down, z2], color_down));

            // Triangle 2
            vertices.push(Vertex::new([wx, h, wz], color));
            vertices.push(Vertex::new([x2, h_down, z2], color_down));
            vertices.push(Vertex::new([wx, fbm(wx * 0.3, z2 * 0.3, 4) * height_scale, z2], color_down));
        }
    }

    let len = vertices.len();
    (vertices, len)
}

struct TerrainApp {
    window: &'static winit::window::Window,
    surface: wgpu::Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    pipeline: RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: usize,
    time: f32,
}

impl TerrainApp {
    async fn new(window: winit::window::Window) -> Self {
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
                    // Camera at (0, 5, 15) looking at origin
                    // Simple perspective transform
                    let aspect = 800.0 / 600.0;
                    let fov = 0.8;
                    let z = input.position.z + 15.0;
                    output.position = vec4<f32>(
                        input.position.x / (z + 5.0) / aspect * fov,
                        (input.position.y - 2.0) / (z + 5.0) * fov,
                        z / 50.0,
                        1.0
                    );
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
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: Default::default(),
            cache: None,
        });

        let (vertices, vertex_count) = create_terrain(64, 20.0);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsages::VERTEX,
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
            time: 0.0,
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

        self.time += 0.016;

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.4,
                            g: 0.5,
                            b: 0.7,
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
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..self.vertex_count as u32, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting rCrabEngine Terrain Example (jME3 HelloTerrain port)");

    let event_loop = EventLoop::new().unwrap();

    #[allow(deprecated)]
    let window = event_loop
        .create_window(winit::window::WindowAttributes::default()
            .with_title("rCrabEngine - Terrain")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600)))
        .unwrap();

    let window_id = window.id();
    let mut app = pollster::block_on(TerrainApp::new(window));

    info!("Procedural terrain with height-based coloring (grass/dirt/snow)");

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
