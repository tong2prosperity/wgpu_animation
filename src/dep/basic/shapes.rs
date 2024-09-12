use super::structure::*;
use wgpu::util::DeviceExt;
use bytemuck;
use super::texture::*;

pub struct FullQuad {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
    pub texture_view: wgpu::TextureView,
}


pub struct FourGradient {
    pub points: [glam::Vec3;4],
    pub point_colors: [glam::Vec4;4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FourGradientUniform {
    pub points: [f32;16],
    pub point_colors: [f32;16],
}



impl FourGradient {
    /* pub fn new() -> Self {
        FourGradient {
            points: [glam::Vec3::new(0.0, 0.0, 0.0); 4],
            point_colors: [glam::Vec4::new(0.0, 0.0, 0.0, 0.0); 4],
        }
    } */

    pub fn random_four_points() -> Self {
        let rng = rand::thread_rng();
        let mut points = [glam::Vec3::new(0.0, 0.0, 0.0); 4];
        let mut point_colors = [glam::Vec4::new(0.0, 0.0, 0.0, 0.0); 4];
        
        
        points[0] = glam::Vec3::new(-0.8, 0.8, 0.0);
        points[1] = glam::Vec3::new(0.4, 0.9, 0.0);
        points[2] = glam::Vec3::new(0.8, -0.4, 0.0);
        points[3] = glam::Vec3::new(-0.7, -0.7, 0.0);

/*         point_colors[0] = glam::Vec4::new(0.2235, 0.2627, 0.7176, 1.0);
        point_colors[1] = glam::Vec4::new(0.7411, 0.9686, 0.7176, 1.0);
        point_colors[2] = glam::Vec4::new(0.9725, 0.9568, 0.6509, 1.0);
        point_colors[3] = glam::Vec4::new(0.4196, 0.1529, 0.2156, 1.0);
         */

        point_colors[0] = glam::Vec4::new(1.0, 0.0, 0.0, 1.0);
        point_colors[1] = glam::Vec4::new(0.0, 1.0, 0.0, 1.0);
        point_colors[2] = glam::Vec4::new(0.0, 0.0, 1.0, 1.0);
        point_colors[3] = glam::Vec4::new(1.0, 1.0, 1.0, 1.0);
         
        FourGradient { points, point_colors }
    }

    pub fn to_uniform(&self) -> FourGradientUniform {
        let mut uniform = FourGradientUniform {
            points: [0.0; 16],
            point_colors: [0.0; 16],
            //padding: 0,
        };

        for i in 0..4 {
            uniform.points[i * 4] = self.points[i].x;
            uniform.points[i * 4 + 1] = self.points[i].y;
            uniform.points[i * 4 + 2] = self.points[i].z;
            uniform.points[i * 4 + 3] = 0.0;
            uniform.point_colors[i * 4] = self.point_colors[i].x;
            uniform.point_colors[i * 4 + 1] = self.point_colors[i].y;
            uniform.point_colors[i * 4 + 2] = self.point_colors[i].z;
            uniform.point_colors[i * 4 + 3] = self.point_colors[i].w;
        }

        uniform
    }
    
}




impl FullQuad {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        // the wgpu full quad
        let vertices = vec![
            Vertex { position: [-1.0, -1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [0.0, 1.0] },
            Vertex { position: [1.0, -1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [0.0, 0.0] },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        let (pp, bg) = Self::create_pipeline(device);
        FullQuad { vertices, indices, pipeline: pp, bind_group: bg, texture_view: create_texture_view(device, config, 1) }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn create_pipeline(device: &wgpu::Device) -> (wgpu::RenderPipeline, wgpu::BindGroup) {

        let four_gradient = FourGradient::random_four_points();
        let uniform = four_gradient.to_uniform();

        let fg_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Four Gradient Uniform Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        let fg_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Four Gradient Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let fg_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Four Gradient Bind Group"),
            layout: &fg_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: fg_buffer.as_entire_binding(),
                }
            ],
        });
        
        
        

        let shader_str = std::fs::read_to_string("./src/res/fourg_shader.wgsl").expect("failed to read shader file");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Full Quad Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_str.into()),
        });


        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Full Quad Pipeline Layout"),
            bind_group_layouts: &[&fg_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Full Quad Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[
                    Vertex::desc(),
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
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
            /* depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24PlusStencil8,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }), */
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        (pipeline, fg_bind_group)
    }
    


    pub fn render(&self, device: &wgpu::Device, texture_view: &wgpu::TextureView) -> wgpu::CommandEncoder {
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Full Quad Encoder"),
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Full Quad Vertex Buffer"),
            contents: bytemuck::cast_slice(&self.vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Full Quad Index Buffer"),
            contents: bytemuck::cast_slice(&self.indices()),
            usage: wgpu::BufferUsages::INDEX,
        });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Full Quad Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            pass.set_bind_group(0, &self.bind_group, &[]);
            
            pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
        }
        encoder
    }
}

pub struct Rectangle {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Rectangle {
    pub fn new(width: f32, height: f32) -> Self {
        let vertices = Self::generate_rectangle_vertices(width, height);
        let indices = Self::generate_rectangle_indices();

        Rectangle { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    fn generate_rectangle_vertices(width: f32, height: f32) -> Vec<Vertex> {
        let aspect_ratio = WIDTH / HEIGHT;

        let vertices = vec![
            Vertex {
                position: [-width / 2.0 / aspect_ratio, height / 2.0, 0.0],
                color: [0.0, 0.0, 0.5],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [width / 2.0 / aspect_ratio, height / 2.0, 0.0],
                color: [1.0, 0.0, 0.5],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [width / 2.0 / aspect_ratio, -height / 2.0, 0.0],
                color: [1.0, 0.0, 0.5],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [-width / 2.0 / aspect_ratio, -height / 2.0, 0.0],
                color: [1.0, 0.0, 0.5],
                uv: [0.0, 0.0],
            },
        ];

        vertices
    }

    fn generate_rectangle_indices() -> Vec<u16> {
        vec![0, 1, 2, 2, 3, 0]
    }
}