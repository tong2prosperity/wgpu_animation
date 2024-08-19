

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

pub const WIDTH: f32 = 480.0;
pub const HEIGHT: f32 = 960.0;

pub(crate) const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-2.0, 2.0, 0.0],
        color: [0.1, 0.0, 0.5],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.2, 0.0, 0.5],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.3, 0.0, 0.5],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.4, 0.0, 0.5],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    }, // E
];

impl Vertex {
    pub fn new(pos: &[f32;3], clr:&[f32;3]) -> Self {
        Vertex {
            position: pos.clone(),
            color: clr.clone(),
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

fn generate_circle_vertices(radius: f32, num_segments: u32) -> Vec<Vertex> {
    let mut vertices = Vec::with_capacity(num_segments as usize + 1);

    // 圆心
    vertices.push(Vertex::new(&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0]));

    // 计算圆周上的顶点
    for i in 0..=num_segments {
        let theta = 2.0 * std::f32::consts::PI * (i as f32) / (num_segments as f32);
        let x_cos  = theta.cos();
        let y_sin  = theta.sin();
        let x = radius * x_cos;
        let y = radius * y_sin;
        vertices.push(Vertex::new(&[x, y, 0.0], &[0.0, 1.0, 0.0]));
    }

    vertices
}

pub const INDICES: &[u16] = &[0, 1, 4,
    1, 2, 4,
    2, 3, 4, /* padding */ 0];