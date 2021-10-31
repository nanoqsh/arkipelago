use crate::{
    program::Program,
    shader::{src, BONES_MAX_LEN},
    uniform::Sampler2d,
    Texture,
};
use glow::Context;
use ngl_derive::{shader_set, uniforms};
use shr::cgm::*;
use std::rc::Rc;

const T0: Sampler2d = Sampler2d::ZERO;

#[shader_set]
pub(crate) struct ShaderSet {
    col: ColorProgram,
    def: DefProgram,
    post: PostProgram,
    skin: SkinProgram,
}

impl ShaderSet {
    pub fn new(ctx: Rc<Context>) -> Self {
        Self::make(
            ColorProgram::new(Program::new(
                Rc::clone(&ctx),
                [src::col_vs(), src::col_fs()],
            )),
            {
                let program = DefProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::def_vs(), src::def_fs()],
                ));

                program.use_program();
                program.set_t0(&T0);
                program
            },
            {
                let program = PostProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::post_vs(), src::post_fs()],
                ));

                program.use_program();
                program.set_t0(&T0);
                program
            },
            {
                let program = SkinProgram::new(Program::new(
                    Rc::clone(&ctx),
                    [src::skin_vs(), src::def_fs()],
                ));

                program.use_program();
                program.set_t0(&T0);
                program
            },
        )
    }

    pub fn bind_texture(&self, tex: &Texture) {
        tex.bind(T0)
    }
}

#[uniforms]
struct ColorProgram {
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

#[uniforms]
struct DefProgram {
    t0: Sampler2d,
    model: Mat4,
    view: Mat4,
    proj: Mat4,
}

#[uniforms]
struct PostProgram {
    t0: Sampler2d,
}

#[uniforms]
struct SkinProgram {
    t0: Sampler2d,
    model: Mat4,
    view: Mat4,
    proj: Mat4,
    bones: [Mat4; BONES_MAX_LEN],
}
