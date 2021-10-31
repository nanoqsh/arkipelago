use crate::uniform::Sampler2d;
use shr::cgm::*;

pub(crate) trait Attribute {
    const SYMBOLIC: u32;
    const ARRAY_LEN: u32 = 1;
}

impl Attribute for i32 {
    const SYMBOLIC: u32 = glow::INT;
}

impl Attribute for u32 {
    const SYMBOLIC: u32 = glow::UNSIGNED_INT;
}

impl Attribute for f32 {
    const SYMBOLIC: u32 = glow::FLOAT;
}

impl Attribute for bool {
    const SYMBOLIC: u32 = glow::BOOL;
}

impl<T, const N: usize> Attribute for [T; N]
where
    T: Attribute,
{
    const SYMBOLIC: u32 = T::SYMBOLIC;
    const ARRAY_LEN: u32 = N as _;
}

impl Attribute for Vec2 {
    const SYMBOLIC: u32 = glow::FLOAT_VEC2;
}

impl Attribute for Vec3 {
    const SYMBOLIC: u32 = glow::FLOAT_VEC3;
}

impl Attribute for Vec4 {
    const SYMBOLIC: u32 = glow::FLOAT_VEC4;
}

impl Attribute for IVec2 {
    const SYMBOLIC: u32 = glow::INT_VEC2;
}

impl Attribute for IVec3 {
    const SYMBOLIC: u32 = glow::INT_VEC3;
}

impl Attribute for IVec4 {
    const SYMBOLIC: u32 = glow::INT_VEC4;
}

impl Attribute for UVec2 {
    const SYMBOLIC: u32 = glow::UNSIGNED_INT_VEC2;
}

impl Attribute for UVec3 {
    const SYMBOLIC: u32 = glow::UNSIGNED_INT_VEC3;
}

impl Attribute for UVec4 {
    const SYMBOLIC: u32 = glow::UNSIGNED_INT_VEC4;
}

impl Attribute for Mat2 {
    const SYMBOLIC: u32 = glow::FLOAT_MAT2;
}

impl Attribute for Mat3 {
    const SYMBOLIC: u32 = glow::FLOAT_MAT3;
}

impl Attribute for Mat4 {
    const SYMBOLIC: u32 = glow::FLOAT_MAT4;
}

impl Attribute for Sampler2d {
    const SYMBOLIC: u32 = glow::SAMPLER_2D;
}

#[derive(Copy, Clone)]
pub(crate) enum Components {
    S1 = 1,
    S2 = 2,
    S3 = 3,
    S4 = 4,
}

pub(crate) trait Component: Attribute {
    const COMPONENTS: Components;
    const ELEMENT_TYPE: u32 = Self::SYMBOLIC;
}

impl Component for i32 {
    const COMPONENTS: Components = Components::S1;
}

impl Component for u32 {
    const COMPONENTS: Components = Components::S1;
}

impl Component for f32 {
    const COMPONENTS: Components = Components::S1;
}

impl Component for bool {
    const COMPONENTS: Components = Components::S1;
    const ELEMENT_TYPE: u32 = glow::BYTE;
}

impl Component for Vec2 {
    const COMPONENTS: Components = Components::S2;
    const ELEMENT_TYPE: u32 = f32::SYMBOLIC;
}

impl Component for Vec3 {
    const COMPONENTS: Components = Components::S3;
    const ELEMENT_TYPE: u32 = f32::SYMBOLIC;
}

impl Component for Vec4 {
    const COMPONENTS: Components = Components::S4;
    const ELEMENT_TYPE: u32 = f32::SYMBOLIC;
}

impl Component for IVec2 {
    const COMPONENTS: Components = Components::S2;
    const ELEMENT_TYPE: u32 = i32::SYMBOLIC;
}

impl Component for IVec3 {
    const COMPONENTS: Components = Components::S3;
    const ELEMENT_TYPE: u32 = i32::SYMBOLIC;
}

impl Component for IVec4 {
    const COMPONENTS: Components = Components::S4;
    const ELEMENT_TYPE: u32 = i32::SYMBOLIC;
}

impl Component for UVec2 {
    const COMPONENTS: Components = Components::S2;
    const ELEMENT_TYPE: u32 = u32::SYMBOLIC;
}

impl Component for UVec3 {
    const COMPONENTS: Components = Components::S3;
    const ELEMENT_TYPE: u32 = u32::SYMBOLIC;
}

impl Component for UVec4 {
    const COMPONENTS: Components = Components::S4;
    const ELEMENT_TYPE: u32 = u32::SYMBOLIC;
}

impl Component for Sampler2d {
    const COMPONENTS: Components = Components::S1;
    const ELEMENT_TYPE: u32 = u32::SYMBOLIC;
}
