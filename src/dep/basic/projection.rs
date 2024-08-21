use glam::*;


pub fn create_ortho_project_matrix() -> Mat4 {
    let width = crate::dep::basic::structure::WIDTH;
    let height = crate::dep::basic::structure::HEIGHT;
    let aspect_ratio = width / height;
    let (left, right)  = (-aspect_ratio, aspect_ratio);
    let bottom = -1.0;
    let top = 1.0;
    let near = -1.0;
    let far = 1.0;
    Mat4::orthographic_rh(left, right, bottom, top, near, far)
}
