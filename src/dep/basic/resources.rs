#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct FeathersUniform {
    pub center: [f32; 2],
    pub radius: f32,
    pub feather: f32,
    pub color: [f32; 4],
}

