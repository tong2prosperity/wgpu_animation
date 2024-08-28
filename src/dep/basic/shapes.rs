use super::structure::*;
use wgpu::util::DeviceExt;
use bytemuck;

pub struct FullQuad {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub pipeline: wgpu::RenderPipeline,
}


pub struct FourGradient {
    pub points: [glam::Vec3;4],
    pub point_colors: [glam::Vec4;4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FourGradientUniform {
    pub points: [f32;12],
    pub point_colors: [f32;16],
}


impl FullQuad {
    pub fn new() -> Self {
        let vertices = vec![
            Vertex { position: [-1.0, -1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [0.0, 1.0] },
            Vertex { position: [1.0, -1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [1.0, 1.0] },
            Vertex { position: [1.0, 1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [1.0, 0.0] },
            Vertex { position: [-1.0, 1.0, 0.0], color: [1.0, 1.0, 1.0], uv: [0.0, 0.0] },
        ];
        let indices = vec![0, 1, 2, 2, 3, 0];
        FullQuad { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    pub fn create_pipeline(device: &wgpu::Device) -> wgpu::RenderPipeline {

        let shader_str = std::fs::read_to_string("./src/res/fourg_shader.wgsl").expect("failed to read shader file");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Full Quad Vertex Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_str.into()),
        });


        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Full Quad Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Full Quad Render Pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: vertex_shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                    
                    ],
                }],
            },
        });

        pipeline
    }
    


    pub fn render(&self, render_pipeline: &wgpu::RenderPipeline, bind_group: &wgpu::BindGroup, device: &wgpu::Device) {
        let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Full Quad Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &self.texture_view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
            })],
            depth_stencil_attachment: None,
        });

        pass.set_pipeline(render_pipeline);
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);

        let _ = pass.finish();
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