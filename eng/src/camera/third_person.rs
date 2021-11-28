use crate::camera::{rotation::Rot, Camera};
use shr::cgm::*;

pub(crate) struct TpCamera {
    cam: Camera,
    rot: Rot,
    distance: f32,
}

impl TpCamera {
    const MIN_DISTANCE: f32 = 0.2;

    pub fn new(distance: f32, look: Pnt3) -> Self {
        Self::with_rot(distance, look, Rot::default().get())
    }

    pub fn with_rot(distance: f32, look: Pnt3, rot: Vec2) -> Self {
        Self {
            cam: Camera::new(Pnt3::origin(), look),
            rot: Rot::new(rot),
            distance: Self::MIN_DISTANCE.max(distance),
        }
    }

    pub fn rot(&self) -> Vec2 {
        self.rot.get()
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn view(&mut self) -> Mat4 {
        let (yaw, pitch) = self.rot().into();
        let (yaw_sin, yaw_cos) = yaw.sin_cos();
        let (pitch_sin, pitch_cos) = pitch.sin_cos();
        let pos = Pnt3::new(yaw_sin * pitch_cos, pitch_sin, yaw_cos * pitch_cos);
        let look = self.cam.look().to_vec();
        self.cam.set_pos(pos * self.distance + look);
        self.cam.view()
    }

    pub fn proj(&self, aspect: f32) -> Mat4 {
        self.cam.proj(aspect)
    }

    pub fn rotate(&mut self, delta: Vec2) {
        self.rot = Rot::new(self.rot() + delta)
    }

    pub fn move_to(&mut self, delta: f32) {
        self.distance = Self::MIN_DISTANCE.max(self.distance + delta);
    }

    pub fn move_look(&mut self, delta: Vec3) {
        self.cam.set_look(self.cam.look() + delta);
    }
}
