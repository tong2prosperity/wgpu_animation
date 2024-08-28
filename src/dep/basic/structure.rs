#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
    pub uv: [f32; 2],
}

pub const WIDTH: f32 = 640.0;
pub const HEIGHT: f32 = 640.0;

pub(crate) const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-2.0, 2.0, 0.0],
        color: [0.1, 0.0, 0.5],
        uv: [0.0, 0.0],
    }, // A
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.2, 0.0, 0.5],
        uv: [0.0, 0.0],
    }, // B
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.3, 0.0, 0.5],
        uv: [0.0, 0.0],
    }, // C
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.4, 0.0, 0.5],
        uv: [0.0, 0.0],
    }, // D
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
        uv: [0.0, 0.0],
    }, // E
];

impl Vertex {
    pub fn new(pos: &[f32; 3], clr: &[f32; 3]) -> Self {
        Vertex {
            position: pos.clone(),
            color: clr.clone(),
            uv: [0.0, 0.0],
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
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}



pub struct Circle {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl Circle {
    pub fn new(center: [f32; 2], radius: f32, segments: u32) -> Self {
        let vertices = Self::generate_circle_vertices(center, radius, segments);
        let indices = Self::generate_circle_indices(vertices.len());

        Circle { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16] {
        &self.indices
    }

    fn generate_circle_vertices(center: [f32; 2], radius: f32, segments: u32) -> Vec<Vertex> {
        let mut vertices = Vec::with_capacity(segments as usize);
        let aspect_ratio = 1.0;

        let step = std::f32::consts::PI * 2.0 / segments as f32;
        vertices.push(Vertex {
            position: [center[0], center[1], 0.0],
            color: [0.0, 1.0, 0.0],
            uv: [0.0, 0.0],
        });

        for i in 0..segments + 1 {
            let angle = step * i as f32;
            let x = center[0] + radius * angle.cos();
            let y = center[1] + radius * angle.sin();
            match i {
                0 .. 30 => {
                    vertices.push(Vertex {
                        position: [x, y, 0.0],
                        color: [1.0, 0.0, 0.0],
                        uv: [0.0, 0.0],
                    });
                }
                30 .. 60 => {
                    vertices.push(Vertex {
                        position: [x, y, 0.0],
                        color: [0.0, 1.0, 0.0],
                        uv: [0.0, 0.0],
                    });
                }
                60 .. 90 => {
                    vertices.push(Vertex {
                        position: [x, y, 0.0],
                        color: [0.0, 0.0, 1.0],
                        uv: [0.0, 0.0],
                    });
                }
                _ => {
                    vertices.push(Vertex {
                        position: [x, y, 0.0],
                        color: [0.5, 0.5, 0.5],
                        uv: [0.0, 0.0],
                    });
                }
            }
        }

        vertices
    }

    fn generate_circle_indices(num_vertices: usize) -> Vec<u16> {
        let mut indices = Vec::with_capacity(num_vertices * 3);

        for i in 0..num_vertices {
            indices.push(0);
            indices.push(i as u16 + 1);
            indices.push((i + 1) as u16 % num_vertices as u16 + 1);
        }

        indices
    }
}

// pub fn generate_circle_vertices(center: [f32; 2], radius: f32, segments: u32) -> Vec<Vertex> {
//     let mut vertices = Vec::with_capacity(segments as usize);
//     let aspect_ratio = WIDTH / HEIGHT;
//
//     // vertices.push(Vertex {
//     //     position: [center[0] / aspect_ratio, center[1], 0.0],
//     //     color: [1.0, 0.0, 0.0], // 默认白色，可以根据需要修改
//     // });
//
//     let step = std::f32::consts::PI * 2.0 / segments as f32;
//     vertices.push(Vertex {
//         position: [center[0] / aspect_ratio, center[1], 0.0],
//         color: [0.0, 0.0, 0.5], // 默认白色,您可以根据需要修改
//     });
//
//     for i in 0..segments + 1 {
//         let angle = step * i as f32;
//         let x = (center[0] + radius * angle.cos()) / aspect_ratio;
//         let y = center[1] + radius * angle.sin();
//
//
//         vertices.push(Vertex {
//             position: [x, y, 0.0],
//             color: [1.0, 0.0, 0.5], // 默认白色,您可以根据需要修改
//         });
//     }
//
//     vertices
// }
//
// pub fn generate_circle_indices(num_vertices: usize) -> Vec<u16> {
//     let mut indices = Vec::with_capacity(num_vertices * 3);
//
//     for i in 0..num_vertices {
//         indices.push(0); // 圆心顶点
//         indices.push(i as u16 + 1); // 当前圆周顶点
//         indices.push((i + 1) as u16 % num_vertices as u16 + 1); // 下一个圆周顶点
//     }
//
//     indices
// }

pub const INDICES: &[u16] = &[0, 1, 4,
    1, 2, 4,
    2, 3, 4, /* padding */ 0];
