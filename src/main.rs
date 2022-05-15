use std::iter;

use crate::render::chunk_buffers::ChunkBuffers;
use crate::render::*;
use cgmath::{Point3, Vector3};
use chunk::Chunk;
use chunk_manager::ChunkManager;
use direction::Direction;
use once_cell::sync::OnceCell;
use rand::prelude::ThreadRng;
use render::texture::Texture;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder}, dpi::{PhysicalPosition, LogicalPosition},
};
use noise::{NoiseFn, Perlin};

mod block_types;
mod chunk;
mod chunk_manager;
mod direction;
mod render;
mod raycaster;
mod math;
mod camera;

pub const RENDER_DISTANCE: i32 = 2;
pub static PERLIN: OnceCell<Perlin> = OnceCell::new();

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    light: f32,
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: (mem::size_of::<[f32; 3]>() + mem::size_of::<[f32; 2]>())
                        as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: wgpu::RenderPipeline,
    pre_render_pipeline: wgpu::RenderPipeline,
    chunk_manager: ChunkManager,
    chunk_buffers: ChunkBuffers,
    #[allow(dead_code)]
    diffuse_texture: texture::Texture,
    diffuse_bind_group: wgpu::BindGroup,
    #[allow(dead_code)]
    sky_diffuse_texture: texture::Texture,
    sky_diffuse_bind_group: wgpu::BindGroup,
    camera: camera::Camera,
    camera_controller: camera::CameraController,
    camera_uniform: camera::CameraUniform,
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    tick: u64,
    pause: bool,
}

impl State {
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &config, "depth_texture");

        let diffuse_bytes = include_bytes!("assets/textures/textures.png");
        let diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            diffuse_bytes,
            "assets/textures/textures.png",
        )
        .unwrap();

        let sky_diffuse_bytes = include_bytes!("assets/textures/sky.png");
        let sky_diffuse_texture = texture::Texture::from_bytes(
            &device,
            &queue,
            sky_diffuse_bytes,
            "assets/textures/sky.png",
        )
        .unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let sky_diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sky_diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sky_diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let camera = camera::Camera {
            pos: (0.0, 1.0, 2.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: config.width as f32 / config.height as f32,
            fovy: 70.0,
            znear: 0.1,
            zfar: 100.0,
            pitch: 0.0,
            yaw: 0.0,
            resolution: window.inner_size(),
        };
        let camera_controller = camera::CameraController::new(0.2);

        let mut camera_uniform = camera::CameraUniform::new();
        camera_uniform.update_view_proj(&camera);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("assets/shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &camera_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        });

        let pre_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sky Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(),     // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut chunk_manager = ChunkManager::new();
        //chunk_manager.add_chunk(Chunk::new_filled(Vector3::new(0, 0, 0), 1));
        //chunk_manager.add_chunk(Chunk::new_layered(Vector3::new(0, 1, 0), 1, 2, 3));
        //chunk_manager.add_chunk(Chunk::new_filled(Vector3::new(-1, 0, 0), 3));
        //chunk_manager.add_chunk(Chunk::new_filled(Vector3::new(-1, 1, 0), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 0, 0), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 1, 0), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 0, 0), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 1, 0), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 0, 0), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 1, 0), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 2, 0), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 2, 0), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 2, 0), 1));

        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 0, 1), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 1, 1), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 0, 1), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 1, 1), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 0, 1), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 1, 1), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 2, 1), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 2, 1), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 2, 1), 1));

        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 0, 2), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 1, 2), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 0, 2), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 1, 2), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 0, 2), 1));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 1, 2), 2));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(-1, 2, 2), 3));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(0, 2, 2), 4));
        chunk_manager.add_chunk(Chunk::new_perlin(Point3::new(1, 2, 2), 1));

        let chunk_buffers = ChunkBuffers::new(&device, &chunk_manager);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            pre_render_pipeline,
            chunk_manager,
            chunk_buffers,
            diffuse_texture,
            diffuse_bind_group,
            sky_diffuse_texture,
            sky_diffuse_bind_group,
            camera,
            camera_controller,
            camera_buffer,
            camera_bind_group,
            camera_uniform,
            depth_texture,
            tick: 0,
            pause: false
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.camera.resolution = new_size;
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);

            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
        }
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
    }

    fn input(&mut self, device_event: Option<&DeviceEvent>, window_event: Option<&WindowEvent>) -> bool {
        self.camera_controller.process_events(device_event, window_event)
    }

    fn update(&mut self) {
        self.tick += 1;

        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );

        if (self.tick % 5) == 0 {
            self.chunk_manager.update(math::pointi32(self.camera.pos), &mut self.chunk_buffers, &self.device);
        }

        /*match raycaster::block_ray(&self.chunk_manager, self.camera.eye, self.camera.target, 0.1, 100.0) {
            Some(hit) => {
                self.chunk_manager.set_block(math::pointi32(hit.position), 0);
                let break_chunk_index = self.chunk_manager.get_pos_index(math::to_vector(math::pointi32(hit.position)));
                let break_chunk = self.chunk_manager.get_pos_chunk(math::pointi32(hit.position));
                if break_chunk_index.is_some() && break_chunk.is_some() {
                    self.chunk_buffers.update_chunk(
                        &self.device,
                        break_chunk_index.unwrap(),
                        break_chunk.unwrap(),
                    );
                }
            },
            None => {},
        };*/
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let (sky0, sky1, sky2) = sky::build(
                &self.device,
                Vector3::new(self.camera.pos.x, self.camera.pos.y, self.camera.pos.z),
            );

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pre_render_pipeline);
            render_pass.set_bind_group(0, &self.sky_diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, sky0.slice(..));
            render_pass.set_index_buffer(sky1.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..sky2, 0, 0..1);

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);

            for buffers in self.chunk_buffers.get_buffers() {
                render_pass.set_vertex_buffer(0, buffers.0.slice(..));
                render_pass.set_index_buffer(buffers.1.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..buffers.2, 0, 0..1);
            }
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    match PERLIN.set(Perlin::new()) {
        Ok(_) => {}
        Err(types) => panic!("Failed to initialize perlin noise!")
    }
    env_logger::init();
    block_types::init();
    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let cursor_pos = PhysicalPosition::new(100, 100);
    
    // State::new uses async code, so we're going to wait for it to finish
    let mut state = pollster::block_on(State::new(&window));

    event_loop.run(move |event, _, control_flow| {
        if !state.pause {
            window.set_cursor_grab(true).unwrap();
            window.set_cursor_visible(false);
        } else {
            window.set_cursor_grab(false).unwrap();
            window.set_cursor_visible(true);
        }
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                state.input(None, Some(&event));
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        state.pause = !state.pause;
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &mut so w have to dereference it twice
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
            Event::DeviceEvent { ref event, .. } => {
                state.input(Some(&event), None);
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                window.request_redraw();
            }
            _ => {}
        }
    });
}
