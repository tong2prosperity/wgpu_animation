use std::f32::consts::PI;
use std::iter;
use super::*;

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowBuilder},
};

use wgpu::{BindGroupEntry, Device, SurfaceConfiguration, TextureDimension, TextureFormat, TextureUsages, TextureView};
use wgpu::util::DeviceExt;
use wgpu::VertexStepMode::Vertex;
use crate::dep::basic::instance::InstanceManager;
use crate::dep::basic::projection::create_ortho_project_matrix;

const SAMPLE_COUNT:u32 =4;

pub struct GPUBuffers {
    pub feather_buffer: wgpu::Buffer,
    pub feather_bg: wgpu::BindGroup,
    pub feather_layout: wgpu::BindGroupLayout,

    pub mvp_buffer : wgpu::Buffer,
    pub mvp_bg : wgpu::BindGroup,
    pub mvp_layout : wgpu::BindGroupLayout,

    pub mat_buffer: wgpu::Buffer,
    pub mat_bg : wgpu::BindGroup,
    pub mat_layout: wgpu::BindGroupLayout
}

pub struct State<'a> {
    #[allow(dead_code)]
    instance: wgpu::Instance,
    #[allow(dead_code)]
    adapter: wgpu::Adapter,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    window: &'a Window,

    instance_manager: InstanceManager,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    texture_view : wgpu::TextureView,
    depth_view : TextureView,
    index_size : usize,
    buffers : GPUBuffers,
    theta : f32,
}

impl<'a> State<'a> {
    pub(crate) fn keyboard_input(&mut self, ke: &KeyEvent) -> bool {
        match ke {
           KeyEvent{
               state: ElementState::Pressed,
               ..
           } => match ke.physical_key{
               PhysicalKey::Code(KeyCode::KeyR) => {
                   self.update_rotate();
                   return true;
               },
               _ => {
               }
           }
            _ => {}
        }
        false
    }

    fn update_rotate(&mut self) {
        let f = PI / 10.0;
        self.theta += f;
        let fmat = create_rotation_matrix(self.theta);
        self.queue.write_buffer(&self.buffers.mat_buffer, 0, bytemuck::cast_slice(&[fmat]));
        println!("you should update here");
    }
}

impl<'a> State<'a> {
    pub async fn new(window: &'a Window) -> State<'a> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = instance.create_surface(window).unwrap();


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
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let surface_caps: wgpu::SurfaceCapabilities = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an Srgb surface texture. Using a different
        // one will result all the colors comming out darker. If you want to support non
        // Srgb surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };


        let multisampled_view = Self::create_texture_view(&device, &config);


        let clear_color = wgpu::Color::BLACK;
        let render_pipeline = Self::create_pipeline(&device, &config);
        let circle = super::structure::Circle::new([0.0,0.0], 0.7, 100);
        //let circle = crate::shapes::circle::generate_circle(0.5);
        //let vert = super::structure::generate_circle_vertices([0.0, 0.0], 0.5, 100);
        //let ind = super::structure::generate_circle_indices(vert.len());
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                //contents: bytemuck::cast_slice(super::structure::VERTICES),
                contents: bytemuck::cast_slice(circle.vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                //contents: bytemuck::cast_slice(super::structure::INDICES),
                contents: bytemuck::cast_slice(circle.indices()),
                usage: wgpu::BufferUsages::INDEX,
            }
        );


        let mut instance_manager = InstanceManager::new();
        instance_manager.make_up_instances();
        instance_manager.init_buffer(&device);

        let depth_view = Self::init_depth_stencil(&device, &config);

         Self {
            instance,
            adapter,
            surface,
            device,
            queue,
            config,
            clear_color,
            size,
            window,
            render_pipeline:render_pipeline.0,
            vertex_buffer,
            index_buffer,
            index_size: circle.indices.len(),
            texture_view: multisampled_view,
            buffers: render_pipeline.1,
             theta: 0.0,
            depth_view,
             instance_manager
        }
    }

    pub fn create_texture_view(device: &Device, config: &SurfaceConfiguration) -> TextureView {
        let multisampled_texture_extent = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };

        let multisampled_texture_desc = wgpu::TextureDescriptor {
            size: multisampled_texture_extent,
            mip_level_count: 1,
            sample_count: 4,
            dimension: wgpu::TextureDimension::D2,
            format: config.format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: Some("Multisampled texture"),
            view_formats: &config.view_formats,
        };

        let multisampled_texture = device.create_texture(&multisampled_texture_desc);
        let multisampled_view = multisampled_texture.create_view(&wgpu::TextureViewDescriptor::default());
        multisampled_view
    }

    pub fn create_pipeline(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> (wgpu::RenderPipeline, GPUBuffers) {
        let buffers = Self::init_uniform(device, config);

        let shader_str = std::fs::read_to_string("./src/res/shader.wgsl").expect("failed to read shader file");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            //source: wgpu::ShaderSource::Wgsl(include_str!("../../res/shader.wgsl").into()),
            source: wgpu::ShaderSource::Wgsl(shader_str.into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &buffers.feather_layout,
                    &buffers.mat_layout,
                    &buffers.mvp_layout
                ],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    super::structure::Vertex::desc(),
                    instance::InstanceRaw::desc(),
                ], // 2.
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                   blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
         //            blend: Some(wgpu::BlendState {
         //                color: wgpu::BlendComponent {
         //                    src_factor: wgpu::BlendFactor::SrcAlpha,
         //                    dst_factor: wgpu::BlendFactor::DstAlpha,
         //                    operation: wgpu::BlendOperation::Add,
         //                },
         //                alpha: wgpu::BlendComponent {
         //                    src_factor: wgpu::BlendFactor::Src,
         //                    dst_factor: wgpu::BlendFactor::Dst,
         //                    operation: wgpu::BlendOperation::Add,
         //                },
         //            }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less, // 1.
                stencil: wgpu::StencilState::default(), // 2.
                bias: wgpu::DepthBiasState::default(),
            }),
            //depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT, // 1.
                mask: !0, // 2.
                alpha_to_coverage_enabled: false, // 3.
            },
            multiview   : None,
            cache    : None,
        }
        );
        (render_pipeline, buffers)
    }

    pub fn init_depth_stencil(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> wgpu::TextureView {
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 4,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth24PlusStencil8,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &config.view_formats,
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        depth_view
    }

    pub fn init_uniform(device: &wgpu::Device, config: &SurfaceConfiguration) -> GPUBuffers{
        let feather = FeathersUniform {
            center: [0.0, 0.0],
            radius: 0.5,
            feather: 0.1,
            color: [0.0, 1.0, 0.0, 1.0],
        };
        let feather_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer feather"),
                contents: bytemuck::cast_slice(&[feather]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );
        
        let feather_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("feather_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer{
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        }
        );

        let feather_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("feather_bind_group"),
            layout: &feather_bind_group_layout,
            entries: &[
                BindGroupEntry{
                    binding: 0,
                    resource: feather_buffer.as_entire_binding(),
                }
            ],
        });

        let mat = create_rotation_matrix(0.0);

        let mat_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer mat"),
                contents: bytemuck::cast_slice(&[mat]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mat_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mat_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::all(),
                    ty: wgpu::BindingType::Buffer{
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let mat_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mat_bind_group"),
            layout: &mat_bind_group_layout,
            entries: &[
                BindGroupEntry{
                    binding: 0,
                    resource: mat_buffer.as_entire_binding(),
                }
            ],
        });


        let mvp = create_ortho_project_matrix((config.width, config.height));


        let mvp_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer mat"),
                contents: bytemuck::cast_slice(&[mvp]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let mvp_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            label: Some("mat_bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry{
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer{
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let mvp_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            label: Some("mat_bind_group"),
            layout: &mvp_bind_group_layout,
            entries: &[
                BindGroupEntry{
                    binding: 0,
                    resource: mvp_buffer.as_entire_binding(),
                }
            ],
        });




        GPUBuffers {
            feather_buffer,
            feather_bg: feather_bind_group,
            feather_layout: feather_bind_group_layout,

            mat_buffer,
            mat_bg: mat_bind_group,
            mat_layout: mat_bind_group_layout,
            mvp_buffer,
            mvp_bg : mvp_bind_group,
            mvp_layout: mvp_bind_group_layout,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let mvp = create_ortho_project_matrix((new_size.width, new_size.height));
            self.queue.write_buffer(&self.buffers.mvp_buffer, 0, bytemuck::cast_slice(&[mvp]));
            let view = Self::create_texture_view(&self.device, &self.config);
            self.texture_view = view;
            self.depth_view = Self::init_depth_stencil(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: position.x as f64 / self.size.width as f64,
                    g: position.y as f64 / self.size.height as f64,
                    b: 1.0,
                    a: 1.0,
                };
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: Some(&view),
                    ops: wgpu::Operations {
                        //load: wgpu::LoadOp::Clear(wgpu::Color{r: 0.0, g:1.0, b:1.0,a:0.0}),
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Discard,
                    },
                })],
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &self.depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(1.0),
                            store: wgpu::StoreOp::Discard,
                        }),
                        stencil_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Clear(0),
                            store: wgpu::StoreOp::Store,
                        })
                    }
                ),
                //depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            _render_pass.set_pipeline(&self.render_pipeline);
            _render_pass.set_bind_group(0, &self.buffers.feather_bg, &[]);
            _render_pass.set_bind_group(1, &self.buffers.mat_bg, &[]);
            _render_pass.set_bind_group(2, &self.buffers.mvp_bg, &[]);
            _render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            _render_pass.set_vertex_buffer(1, self.instance_manager.get_buffer().slice(..));
            _render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            _render_pass.draw_indexed(0..self.index_size as u32, 0, 0..self.instance_manager.instances.len() as u32);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
