use shr::cgm::*;
use std::f32::consts::PI;

pub struct Camera {
    pos: Pnt3,
    look: Pnt3,
    zoom: Rad,
}

impl Camera {
    const DEFAULT_ZOOM: f32 = PI / 2.5;
    const MIN_ZOOM: f32 = 0.1;
    const MAX_ZOOM: f32 = PI - 0.1;
    const NEAR: f32 = 0.1;
    const FAR: f32 = 100.;

    pub fn new(pos: Pnt3, look: Pnt3) -> Self {
        Self {
            pos,
            look,
            zoom: Self::DEFAULT_ZOOM.rad(),
        }
    }

    pub fn pos(&self) -> Pnt3 {
        self.pos
    }

    pub fn look(&self) -> Pnt3 {
        self.look
    }

    pub fn zoom(&self) -> Rad {
        self.zoom
    }

    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.look, Vec3::unit_y())
    }

    pub fn proj(&self, aspect: f32) -> Mat4 {
        cgmath::PerspectiveFov {
            fovy: self.zoom,
            aspect,
            near: Self::NEAR,
            far: Self::FAR,
        }
        .into()
    }

    pub fn set_pos(&mut self, pos: Pnt3) {
        self.pos = pos;
    }

    pub fn set_look(&mut self, look: Pnt3) {
        self.look = look
    }

    pub fn set_zoom(&mut self, zoom: Rad) {
        self.zoom = zoom.0.clamp(Self::MIN_ZOOM, Self::MAX_ZOOM).rad();
    }
}
