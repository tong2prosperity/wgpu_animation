use glam::*;

pub fn create_ortho_project_matrix(size: (u32, u32)) -> Mat4 {
    let width = size.0 as f32;
    let height = size.1 as f32;
    let aspect_ratio = width / height;
    let (left, right)  = (-aspect_ratio, aspect_ratio);
    let bottom = -1.0;
    let top = 1.0;
    let near = -1.0;
    let far = 1.0;
    Mat4::orthographic_rh(left, right, bottom, top, near, far)
}
