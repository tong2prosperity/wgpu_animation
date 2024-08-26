

struct Instance {
    pub position: glam::Vec3,
    pub rotation: glam::Mat3,
    pub transform: glam::Mat3,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub model: [[f32;4];4],
}

impl Instance {
    pub fn new(position: glam::Vec3, rotation: glam::Mat3) -> Self {
        let transform = glam::Mat3::rotation_y() * glam::Mat3::from_translation(position);
        Self { position, rotation, transform }
    }

    pub fn desc() -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}