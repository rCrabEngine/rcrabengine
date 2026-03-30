// Hello Multi-Light - Multiple colored lights orbiting a sphere
// Port of jME3 TestSimpleLighting - demonstrates point lights and directional lights

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
    normal: [f32; 3],
}

impl Vertex {
    fn new(position: [f32; 3], normal: [f32; 3]) -> Self {
        Self { position, normal }
    }
}

// Simple sphere vertex generator
fn create_sphere(radius: f32, segments: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    for lat in 0..segments {
        let theta1 = (lat as f32) * std::f32::consts::PI / segments as f32;
        let theta2 = ((lat + 1) as f32) * std::f32::consts::PI / segments as f32;

        for lon in 0..segments {
            let phi1 = (lon as f32) * 2.0 * std::f32::consts::PI / segments as f32;
            let phi2 = ((lon + 1) as f32) * 2.0 * std::f32::consts::PI / segments as f32;

            // Four vertices of the quad
            let p11 = sphere_point(radius, theta1, phi1);
            let p21 = sphere_point(radius, theta1, phi2);
            let p12 = sphere_point(radius, theta2, phi1);
            let p22 = sphere_point(radius, theta2, phi2);

            // Two triangles
            vertices.push(Vertex::new(p11.0, p11.1));
            vertices.push(Vertex::new(p21.0, p21.1));
            vertices.push(Vertex::new(p22.0, p22.1));

            vertices.push(Vertex::new(p11.0, p11.1));
            vertices.push(Vertex::new(p22.0, p22.1));
            vertices.push(Vertex::new(p12.0, p12.1));
        }
    }

    vertices
}

fn sphere_point(radius: f32, theta: f32, phi: f32) -> ([f32; 3], [f32; 3]) {
    let x = radius * theta.sin() * phi.cos();
    let y = radius * theta.cos();
    let z = radius * theta.sin() * phi.sin();
    let normal = [x / radius, y / radius, z / radius];
    ([x, y, z], normal)
}

struct MultiLightApp {
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

impl MultiLightApp {
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

        // Shader with simple per-vertex lighting
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(
                r#"
                struct VertexInput {
                    @location(0) position: vec3<f32>,
                    @location(1) normal: vec3<f32>,
                }

                struct VertexOutput {
                    @builtin(position) position: vec4<f32>,
                    @location(0) normal: vec3<f32>,
                    @location(1) world_pos: vec3<f32>,
                }

                // Light positions and colors (hardcoded for 3 lights)
                // Light 0: White point light orbiting
                // Light 1: Red directional light
                // Light 2: Blue point light
                // Also simple diffuse shading

                @vertex
                fn vs_main(input: VertexInput) -> VertexOutput {
                    var output: VertexOutput;
                    // Move sphere back a bit
                    let world_pos = input.position + vec3<f32>(0.0, 0.0, -3.0);
                    output.position = vec4<f32>(world_pos.x * 0.4, world_pos.y * 0.4, world_pos.z * 0.4 + 0.5, 1.0);
                    output.normal = input.normal;
                    output.world_pos = world_pos;
                    return output;
                }

                @fragment
                fn fs_main(@location(0) normal: vec3<f32>, @location(1) world_pos: vec3<f32>) -> @location(0) vec4<f32> {
                    // Base color (gray)
                    var base_color = vec3<f32>(0.7, 0.7, 0.7);

                    // Light 0: White point light orbiting
                    let angle0 = 2.0;
                    let light0_pos = vec3<f32>(cos(angle0) * 2.0, 0.5, sin(angle0) * 2.0 - 3.0);
                    let to_light0 = light0_pos - world_pos;
                    let dist0 = length(to_light0);
                    let atten0 = 1.0 / (1.0 + 0.5 * dist0 * dist0);
                    let diff0 = max(dot(normalize(normal), normalize(to_light0)), 0.0);
                    let light0 = vec3<f32>(1.0, 1.0, 1.0) * diff0 * atten0;

                    // Light 1: Green directional light
                    let light1_dir = normalize(vec3<f32>(-1.0, -1.0, -1.0));
                    let diff1 = max(dot(normalize(normal), light1_dir), 0.0);
                    let light1 = vec3<f32>(0.0, 1.0, 0.0) * diff1 * 0.5;

                    // Light 2: Red point light
                    let light2_pos = vec3<f32>(2.0, 2.0, -2.0);
                    let to_light2 = light2_pos - world_pos;
                    let dist2 = length(to_light2);
                    let atten2 = 1.0 / (1.0 + 0.3 * dist2 * dist2);
                    let diff2 = max(dot(normalize(normal), normalize(to_light2)), 0.0);
                    let light2 = vec3<f32>(1.0, 0.0, 0.3) * diff2 * atten2;

                    // Ambient
                    let ambient = vec3<f32>(0.1, 0.1, 0.15);

                    let final_color = base_color * (ambient + light0 + light1 + light2);

                    return vec4<f32>(final_color, 1.0);
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
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            depth_stencil: None,
            multisample: Default::default(),
            multiview_mask: Default::default(),
            cache: None,
        });

        let vertices = create_sphere(1.0, 32);
        let vertex_count = vertices.len();

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
                            r: 0.05,
                            g: 0.05,
                            b: 0.08,
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

    info!("Starting rCrabEngine Multi-Light Example (jME3 TestSimpleLighting port)");

    let event_loop = EventLoop::new().unwrap();

    #[allow(deprecated)]
    let window = event_loop
        .create_window(winit::window::WindowAttributes::default()
            .with_title("rCrabEngine - Multi-Light")
            .with_inner_size(winit::dpi::LogicalSize::new(800, 600)))
        .unwrap();

    let window_id = window.id();
    let mut app = pollster::block_on(MultiLightApp::new(window));

    info!("Sphere with 3 lights: White orbiting, Green directional, Red point");

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
