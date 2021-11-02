use crate::uniform::Sampler2d;
use glow::{Context, HasContext, NativeTexture};
use shr::cgm::*;
use std::rc::Rc;

pub struct Texture {
    nat: NativeTexture,
    ctx: Rc<Context>,
    typ: Type,
    size: UVec2,
}

impl Texture {
    pub(crate) fn empty(ctx: Rc<Context>, size: UVec2, params: Parameters) -> Self {
        Self::new(ctx, &[], size, params)
    }

    pub(crate) fn new(ctx: Rc<Context>, data: &[u8], size: UVec2, params: Parameters) -> Self {
        let Parameters {
            typ,
            format,
            filter,
            wrap,
        } = params;

        unsafe {
            let nat = ctx.create_texture().expect("create texture");
            Self::make(&ctx, nat, data, size, typ, format);

            if let Type::Common = typ {
                let gl = wrap.gl() as _;
                ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, gl);
                ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, gl);

                let gl = filter.gl() as _;
                ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, gl);
                ctx.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, gl);
            }

            Self {
                nat,
                ctx,
                typ,
                size,
            }
        }
    }

    pub(crate) fn nat(&self) -> NativeTexture {
        self.nat
    }

    pub(crate) fn typ(&self) -> Type {
        self.typ
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }

    pub fn resize(&mut self, data: &[u8], size: UVec2, format: Format) {
        Self::make(&self.ctx, self.nat, data, size, self.typ, format);
        self.size = size;
    }

    pub(crate) fn bind(&self, unit: Sampler2d) {
        unsafe {
            self.ctx.active_texture(unit.gl());
            self.ctx.bind_texture(self.typ.gl(), Some(self.nat));
        }
    }

    fn make(
        ctx: &Context,
        nat: NativeTexture,
        data: &[u8],
        size: UVec2,
        typ: Type,
        format: Format,
    ) {
        let (width, height) = size.into();
        assert!(width <= glow::MAX_TEXTURE_SIZE);
        assert!(height <= glow::MAX_TEXTURE_SIZE);

        let pixels = (!data.is_empty()).then(|| {
            let req = (width as usize * height as usize) * format.n_components();
            if data.len() != req {
                panic!("wrong data length");
            }

            data
        });

        unsafe {
            let gl_target = typ.gl();
            ctx.bind_texture(gl_target, Some(nat));
            match typ {
                Type::Common => ctx.tex_image_2d(
                    gl_target,
                    0,
                    format.gl() as _,
                    width as _,
                    height as _,
                    0,
                    format.gl(),
                    glow::UNSIGNED_BYTE,
                    pixels,
                ),
                Type::Multisample(samples) => ctx.tex_image_2d_multisample(
                    gl_target,
                    {
                        assert_ne!(samples, 0);
                        assert!(samples as u32 <= glow::MAX_INTEGER_SAMPLES);
                        samples as _
                    },
                    format.gl() as _,
                    width as _,
                    height as _,
                    true,
                ),
            }
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.ctx.bind_texture(self.typ.gl(), None);
            self.ctx.delete_texture(self.nat);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Parameters {
    pub typ: Type,
    pub format: Format,
    pub filter: Filter,
    pub wrap: Wrap,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            typ: Type::Common,
            format: Format::Rgba,
            filter: Filter::Nearest,
            wrap: Wrap::Repeat,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Type {
    Common,
    Multisample(u8),
}

impl Type {
    pub(crate) fn gl(self) -> u32 {
        match self {
            Self::Common => glow::TEXTURE_2D,
            Self::Multisample(_) => glow::TEXTURE_2D_MULTISAMPLE,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Format {
    R,
    Rg,
    Rgb,
    Rgba,
}

impl Format {
    fn gl(self) -> u32 {
        match self {
            Self::R => glow::RED,
            Self::Rg => glow::RG,
            Self::Rgb => glow::RGB,
            Self::Rgba => glow::RGBA,
        }
    }

    fn n_components(self) -> usize {
        match self {
            Self::R => 1,
            Self::Rg => 2,
            Self::Rgb => 3,
            Self::Rgba => 4,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Filter {
    Nearest,
    Linear,
}

impl Filter {
    fn gl(self) -> u32 {
        match self {
            Self::Nearest => glow::NEAREST,
            Self::Linear => glow::LINEAR,
        }
    }
}

#[derive(Copy, Clone)]
pub enum Wrap {
    Repeat,
    ClampToEdge,
    ClampToBorder,
    MirroredRepeat,
    MirrorClampToEdge,
}

impl Wrap {
    fn gl(self) -> u32 {
        match self {
            Self::Repeat => glow::REPEAT,
            Self::ClampToEdge => glow::CLAMP_TO_EDGE,
            Self::ClampToBorder => glow::CLAMP_TO_BORDER,
            Self::MirroredRepeat => glow::MIRRORED_REPEAT,
            Self::MirrorClampToEdge => glow::MIRROR_CLAMP_TO_EDGE,
        }
    }
}
