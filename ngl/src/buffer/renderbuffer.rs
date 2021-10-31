use glow::{Context, HasContext, NativeRenderbuffer};
use shr::cgm::*;
use std::rc::Rc;

pub(crate) struct Renderbuffer {
    nat: NativeRenderbuffer,
    ctx: Rc<Context>,
}

impl Renderbuffer {
    pub fn new(ctx: Rc<Context>, size: UVec2, format: Format) -> Self {
        unsafe {
            let nat = ctx.create_renderbuffer().expect("create renderbuffer");
            Self::make(&ctx, nat, size, format);

            Self { nat, ctx }
        }
    }

    pub fn resize(&mut self, size: UVec2, format: Format) {
        Self::make(&self.ctx, self.nat, size, format)
    }

    pub(crate) fn nat(&self) -> NativeRenderbuffer {
        self.nat
    }

    fn make(ctx: &Context, nat: NativeRenderbuffer, size: UVec2, format: Format) {
        let (width, height) = size.into();
        assert!(width <= glow::MAX_RENDERBUFFER_SIZE);
        assert!(height <= glow::MAX_RENDERBUFFER_SIZE);

        unsafe {
            ctx.bind_renderbuffer(glow::RENDERBUFFER, Some(nat));
            ctx.renderbuffer_storage(glow::RENDERBUFFER, format.gl(), width as _, height as _)
        }
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_renderbuffer(self.nat) }
    }
}

#[derive(Copy, Clone)]
pub(crate) enum Format {
    Depth16,
    Depth24,
    DepthF32,
    Depth24Stencil8,
    DepthF32Stencil8,
    Stencil8,
}

impl Format {
    fn gl(self) -> u32 {
        match self {
            Self::Depth16 => glow::DEPTH_COMPONENT16,
            Self::Depth24 => glow::DEPTH_COMPONENT24,
            Self::DepthF32 => glow::DEPTH_COMPONENT32F,
            Self::Depth24Stencil8 => glow::DEPTH24_STENCIL8,
            Self::DepthF32Stencil8 => glow::DEPTH32F_STENCIL8,
            Self::Stencil8 => glow::STENCIL_INDEX8,
        }
    }

    pub(crate) fn gl_attachment(self) -> u32 {
        match self {
            Self::Depth16 | Self::Depth24 | Self::DepthF32 => glow::DEPTH_ATTACHMENT,
            Self::Depth24Stencil8 | Self::DepthF32Stencil8 => glow::DEPTH_STENCIL_ATTACHMENT,
            Self::Stencil8 => glow::STENCIL_ATTACHMENT,
        }
    }
}
