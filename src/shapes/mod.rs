pub mod circle;

pub struct Color {
    data : glam::Vec3
}


pub struct Shadow {
    pub color: Color,
    pub offset: (f32, f32),
    pub blur_radius: f32,
}
pub trait Shape {
    fn set_color(&mut self, color: Color);
    fn set_shadow(&mut self, shadow: Shadow);
    fn draw(&self, render_pass: &mut wgpu::RenderPass);
}
