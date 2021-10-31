use crate::camera::{rotation::Rot, Camera};
use shr::cgm::*;

pub(crate) struct FpCamera {
    cam: Camera,
    rot: Rot,
}

impl FpCamera {
    pub fn new() -> Self {
        Self::with_rot(Rot::default().get())
    }

    pub fn with_rot(rot: Vec2) -> Self {
        Self {
            cam: Camera::new(Pnt3::origin(), Pnt3::new(0., 0., 1.)),
            rot: Rot::new(rot),
        }
    }

    pub fn rot(&self) -> Vec2 {
        self.rot.get()
    }

    pub fn view(&mut self) -> Mat4 {
        let (yaw, pitch) = self.rot().into();
        let (yaw_sin, yaw_cos) = yaw.sin_cos();
        let (pitch_sin, pitch_cos) = pitch.sin_cos();
        let look = Vec3::new(yaw_sin * pitch_cos, pitch_sin, yaw_cos * pitch_cos);
        self.cam.set_look(self.cam.pos() + look);
        self.cam.view()
    }

    pub fn proj(&self, aspect: f32) -> Mat4 {
        self.cam.proj(aspect)
    }

    pub fn rotate(&mut self, delta: Vec2) {
        self.rot = Rot::new(self.rot() + delta)
    }
}
