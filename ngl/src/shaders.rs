use crate::{
    program::Program,
    shader::{src, BONES_MAX_LEN},
    uniform::Sampler2d,
};
use glow::Context;
use ngl_derive::uniforms;
use shr::cgm::*;
use std::rc::Rc;

pub(crate) struct Shaders {
    pub solid: SolidProgram,
    pub skin: SkinProgram,
    pub color: ColorProgram,
    pub post: PostProgram,
}

impl Shaders {
    pub const T0: Sampler2d = Sampler2d::ZERO;

    pub fn new(ctx: Rc<Context>) -> Self {
        Self {
            solid: {
                let program = SolidProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::def_vs(), src::def_fs()],
                ));

                program.use_program();
                program.set_t0(&Self::T0);
                program
            },
            skin: {
                let program = SkinProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::skin_vs(), src::def_fs()],
                ));

                program.use_program();
                program.set_t0(&Self::T0);
                program
            },
            color: {
                ColorProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::col_vs(), src::col_fs()],
                ))
            },
            post: {
                let program = PostProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::post_vs(), src::post_fs()],
                ));

                program.use_program();
                program.set_t0(&Self::T0);
                program
            },
        }
    }
}

#[uniforms]
pub(crate) struct SolidProgram {
    t0: Sampler2d,
    fog_cl: Vec3,
    fog_near: f32,
    fog_far: f32,
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

#[uniforms]
pub(crate) struct SkinProgram {
    t0: Sampler2d,
    fog_cl: Vec3,
    fog_near: f32,
    fog_far: f32,
    model: Mat4,
    view: Mat4,
    proj: Mat4,
    bones: [Mat4; BONES_MAX_LEN],
}

#[uniforms]
pub(crate) struct ColorProgram {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

#[uniforms]
pub(crate) struct PostProgram {
    t0: Sampler2d,
    vignette_cl: Vec3,
}
