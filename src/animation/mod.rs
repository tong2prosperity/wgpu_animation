
use super::shapes::*;
pub trait Animator {
    fn update(&mut self, delta_time: f32);
    fn apply(&self, shape: &mut dyn Shape);
}


pub struct RotationAnimator {
    angle: f32,
    speed: f32,
}

impl Animator for RotationAnimator {
    fn update(&mut self, delta_time: f32) {
        self.angle += self.speed * delta_time;
    }

    fn apply(&self, shape: &mut dyn Shape) {
        // 应用旋转角度到图形
    }
}
