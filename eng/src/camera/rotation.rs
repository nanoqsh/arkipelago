use shr::cgm::*;
use std::f32::consts::{FRAC_PI_2, TAU};

#[derive(Copy, Clone)]
pub(crate) struct Rot {
    yaw: Rad,
    pitch: Rad,
}

impl Rot {
    const DEFAULT_YAW: f32 = 0.;
    const DEFAULT_PITCH: f32 = 0.;
    const MIN_PITCH: f32 = -FRAC_PI_2 + 0.1;
    const MAX_PITCH: f32 = FRAC_PI_2 - 0.1;

    pub fn new(rot: Vec2) -> Self {
        let yaw = (rot.x % TAU).rad();
        let pitch = rot.y.clamp(Self::MIN_PITCH, Self::MAX_PITCH).rad();
        Self { yaw, pitch }
    }

    pub fn get(self) -> Vec2 {
        Vec2::new(self.yaw.0, self.pitch.0)
    }
}

impl Default for Rot {
    fn default() -> Self {
        Self::new(Vec2::new(Self::DEFAULT_YAW, Self::DEFAULT_PITCH))
    }
}
