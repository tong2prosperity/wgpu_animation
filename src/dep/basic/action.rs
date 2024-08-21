use nalgebra::{Matrix3, Rotation3, Vector3};
use nalgebra::Rotation2;
use glam::Vec3;


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ActionMatrix {
    pub theta: f32,
    pub mat : [f32;9],
    pub padding: [f32;6]
}

pub fn create_rotation_matrix(angle: f32) -> ActionMatrix {
    let y_axis = Vector3::y_axis();
    let rotation = Rotation3::from_axis_angle(&y_axis, angle);

    let mat = Matrix3::from(rotation);
    let copy = mat.as_slice();
    let mut k = [0.0f32;9];

    for i in 0..9 {
        k[i] = copy[i];
    }


    ActionMatrix {
        theta: angle,
        mat: k,
        padding: [0.0f32;6]
    }
}

// pub fn create_glam_matrix(angle: f32) -> ActionMatrix {
//     let mat = glam::Mat2::from_angle();
//
// }
