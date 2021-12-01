use glow::{Context, HasContext, NativeUniformLocation};
use shr::cgm::*;

pub(crate) trait Uniform {
    /// Sets uniform value.
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation);

    /// Sets uniform slice.
    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        for val in slice {
            val.uniform(ctx, loc)
        }
    }
}

impl Uniform for i32 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_i32(Some(&loc), *self) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_i32_slice(Some(&loc), slice) }
    }
}

impl Uniform for u32 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_u32(Some(&loc), *self) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_u32_slice(Some(&loc), slice) }
    }
}

impl Uniform for f32 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_f32(Some(&loc), *self) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_1_f32_slice(Some(&loc), slice) }
    }
}

impl Uniform for bool {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        (*self as i32).uniform(ctx, loc)
    }
}

impl<T: Uniform, const N: usize> Uniform for [T; N] {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        T::uniform_slice(self, ctx, loc)
    }
}

impl<T: Uniform> Uniform for [T] {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        T::uniform_slice(self, ctx, loc)
    }
}

impl Uniform for Vec2 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_2_f32(Some(&loc), self.x, self.y) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 2);
            ctx.uniform_2_f32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for Vec3 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_3_f32(Some(&loc), self.x, self.y, self.z) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 3);
            ctx.uniform_3_f32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for Vec4 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_4_f32(Some(&loc), self.x, self.y, self.z, self.w) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 4);
            ctx.uniform_4_f32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for IVec2 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_2_i32(Some(&loc), self.x, self.y) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 2);
            ctx.uniform_2_i32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for IVec3 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_3_i32(Some(&loc), self.x, self.y, self.z) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 3);
            ctx.uniform_3_i32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for IVec4 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_4_i32(Some(&loc), self.x, self.y, self.z, self.w) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 4);
            ctx.uniform_4_i32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for UVec2 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_2_u32(Some(&loc), self.x, self.y) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 2);
            ctx.uniform_2_u32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for UVec3 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_3_u32(Some(&loc), self.x, self.y, self.z) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 3);
            ctx.uniform_3_u32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for UVec4 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        unsafe { ctx.uniform_4_u32(Some(&loc), self.x, self.y, self.z, self.w) }
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), slice.len() * 4);
            ctx.uniform_4_u32_slice(Some(&loc), slice)
        }
    }
}

impl Uniform for Mat2 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        Self::uniform_slice(std::slice::from_ref(self), ctx, loc)
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), 2 * 2 * slice.len());
            ctx.uniform_matrix_2_f32_slice(Some(&loc), false, slice);
        }
    }
}

impl Uniform for Mat3 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        Self::uniform_slice(std::slice::from_ref(self), ctx, loc)
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), 3 * 3 * slice.len());
            ctx.uniform_matrix_3_f32_slice(Some(&loc), false, slice);
        }
    }
}

impl Uniform for Mat4 {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        Self::uniform_slice(std::slice::from_ref(self), ctx, loc)
    }

    fn uniform_slice(slice: &[Self], ctx: &Context, loc: NativeUniformLocation)
    where
        Self: Sized,
    {
        unsafe {
            let slice = std::slice::from_raw_parts(slice.as_ptr().cast(), 4 * 4 * slice.len());
            ctx.uniform_matrix_4_f32_slice(Some(&loc), false, slice);
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Sampler2d(i32);

impl Sampler2d {
    pub const ZERO: Self = Self(0);

    #[allow(dead_code)]
    pub const fn new(unit: i32) -> Option<Self> {
        const MAX: i32 = glow::MAX_COMBINED_TEXTURE_IMAGE_UNITS as i32 - 1;

        match unit {
            0..=MAX => Some(Self(unit)),
            _ => None,
        }
    }

    pub const fn get(self) -> i32 {
        self.0
    }

    pub const fn gl(self) -> u32 {
        glow::TEXTURE0 + self.0 as u32
    }
}

impl Uniform for Sampler2d {
    fn uniform(&self, ctx: &Context, loc: NativeUniformLocation) {
        self.0.uniform(ctx, loc)
    }
}
