use crate::uniform::Sampler2d;
use shr::cgm::*;
use std::fmt::Write;

pub(crate) trait Declare {
    /// Prints glsl type
    fn declare_type(out: &mut String)
    where
        Self: Sized;

    /// Prints literal value
    fn literal(&self, out: &mut String);
}

impl Declare for i32 {
    fn declare_type(out: &mut String) {
        write!(out, "int").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "{self}").unwrap()
    }
}

impl Declare for u32 {
    fn declare_type(out: &mut String) {
        write!(out, "uint").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "{self}u").unwrap()
    }
}

impl Declare for f32 {
    fn declare_type(out: &mut String) {
        write!(out, "float").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "{self}").unwrap();

        if self.fract() == 0. {
            write!(out, ".0",).unwrap();
        }
    }
}

impl Declare for bool {
    fn declare_type(out: &mut String) {
        write!(out, "bool").unwrap()
    }

    fn literal(&self, out: &mut String) {
        let lit = if *self { "true" } else { "false" };
        write!(out, "{lit}").unwrap()
    }
}

impl<T: Declare, const N: usize> Declare for [T; N] {
    fn declare_type(out: &mut String) {
        T::declare_type(out);
        write!(out, "[{N}]").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "{{").unwrap();

        if let [head, tail @ ..] = &self[..] {
            head.literal(out);
            for v in tail {
                write!(out, ", ").unwrap();
                v.literal(out);
            }
        }

        write!(out, "}}").unwrap();
    }
}

impl Declare for Vec2 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "vec2").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "vec2(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Vec3 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "vec3").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "vec3(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Vec4 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "vec4").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "vec4(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ", ").unwrap();
        self.w.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for IVec2 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "ivec2").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "ivec2(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for IVec3 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "ivec3").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "ivec3(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for IVec4 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "ivec4").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "ivec4(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ", ").unwrap();
        self.w.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for UVec2 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "uvec2").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "uvec2(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for UVec3 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "uvec3").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "uvec3(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for UVec4 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "uvec4").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "uvec4(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ", ").unwrap();
        self.w.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Mat2 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "mat2").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "mat2(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Mat3 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "mat3").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "mat3(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Mat4 {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "mat4").unwrap()
    }

    fn literal(&self, out: &mut String) {
        write!(out, "mat4(").unwrap();
        self.x.literal(out);
        write!(out, ", ").unwrap();
        self.y.literal(out);
        write!(out, ", ").unwrap();
        self.z.literal(out);
        write!(out, ", ").unwrap();
        self.w.literal(out);
        write!(out, ")").unwrap();
    }
}

impl Declare for Sampler2d {
    fn declare_type(out: &mut String)
    where
        Self: Sized,
    {
        write!(out, "sampler2D").unwrap()
    }

    fn literal(&self, out: &mut String) {
        self.get().literal(out)
    }
}
