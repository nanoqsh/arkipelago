use crate::layout::Layout;
use shr::cgm::*;

#[derive(Layout)]
pub struct ColorVertex {
    pub co: Vec3,
    pub cl: Vec3,
}

#[derive(Layout)]
pub struct Vertex {
    pub co: Vec3,
    pub nm: Vec3,
    pub st: Vec2,
}

#[derive(Layout)]
pub struct PostVertex {
    pub co: Vec2,
    pub st: Vec2,
}

#[derive(Layout)]
pub struct SkinVertex {
    pub co: Vec3,
    pub nm: Vec3,
    pub st: Vec2,
    pub bs: UVec3,
    pub ws: Vec3,
}
