use super::structure::*;


pub struct FullQuad {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
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