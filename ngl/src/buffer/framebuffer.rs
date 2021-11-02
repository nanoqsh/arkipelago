use crate::buffer::{
    renderbuffer::{self, Renderbuffer},
    texture::{self, Texture},
};
use glow::{Context, HasContext, NativeFramebuffer};
use shr::cgm::*;
use std::rc::Rc;

struct Rb {
    rb: Renderbuffer,
    format: renderbuffer::Format,
}

struct Tx {
    tex: Texture,
    format: texture::Format,
}

pub(crate) struct Framebuffer {
    nat: NativeFramebuffer,
    ctx: Rc<Context>,
    renderbuffer: Option<Rb>,
    textures: Vec<Tx>,
}

impl Framebuffer {
    pub fn new(ctx: Rc<Context>, attachments: Attachments, size: UVec2, samples: u8) -> Self {
        unsafe {
            let nat = ctx.create_framebuffer().expect("create framebuffer");
            ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(nat));
            let (renderbuffer, textures) = attachments.attach(Rc::clone(&ctx), size, samples);

            if let Err(err) = Self::check_completion(&ctx) {
                panic!("framebuffer is not completed: {:?}", err)
            }

            Self {
                nat,
                ctx,
                renderbuffer,
                textures,
            }
        }
    }

    pub fn resize(&mut self, size: UVec2) {
        if let Some(Rb { rb, format }) = &mut self.renderbuffer {
            rb.resize(size, *format)
        }

        for Tx { tex, format } in &mut self.textures {
            tex.resize(&[], size, *format)
        }
    }

    pub fn nat(&self) -> NativeFramebuffer {
        self.nat
    }

    pub fn bind(&self) {
        unsafe { self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.nat)) }
    }

    pub fn bind_default(&self) {
        unsafe { self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None) }
    }

    pub fn texture(&self, idx: usize) -> Option<&Texture> {
        self.textures.get(idx).map(|Tx { tex, .. }| tex)
    }

    pub fn blit_to(&self, rhs: &Self, size: UVec2) {
        let (width, height) = size.into();

        unsafe {
            self.ctx
                .bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.nat));
            self.ctx
                .bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(rhs.nat));
            self.ctx.blit_framebuffer(
                0,
                0,
                width as _,
                height as _,
                0,
                0,
                width as _,
                height as _,
                glow::COLOR_BUFFER_BIT,
                glow::NEAREST,
            )
        }
    }

    fn check_completion(ctx: &Context) -> Result<(), CompletionError> {
        match unsafe { ctx.check_framebuffer_status(glow::FRAMEBUFFER) } {
            glow::FRAMEBUFFER_COMPLETE => Ok(()),
            err => Err(CompletionError::from_error(err)),
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_framebuffer(self.nat) }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Attachments<'a> {
    pub renderbuffer: Option<renderbuffer::Format>,
    pub textures: &'a [texture::Format],
}

impl Attachments<'_> {
    fn attach(self, ctx: Rc<Context>, size: UVec2, samples: u8) -> (Option<Rb>, Vec<Tx>) {
        let renderbuffer = self.renderbuffer.map(|format| unsafe {
            let params = renderbuffer::Parameters {
                typ: match samples {
                    0 => renderbuffer::Type::Common,
                    _ => renderbuffer::Type::Multisample(samples),
                },
                format,
            };

            let rb = Renderbuffer::new(Rc::clone(&ctx), size, params);
            ctx.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                format.gl_attachment(),
                glow::RENDERBUFFER,
                Some(rb.nat()),
            );

            Rb { rb, format }
        });

        let textures = self
            .textures
            .iter()
            .enumerate()
            .map(|(n, &format)| unsafe {
                let typ = match samples {
                    0 => texture::Type::Common,
                    _ => texture::Type::Multisample(samples),
                };

                let params = texture::Parameters {
                    typ,
                    format,
                    ..texture::Parameters::default()
                };

                let tex = Texture::empty(Rc::clone(&ctx), size, params);
                ctx.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    glow::COLOR_ATTACHMENT0 + n as u32,
                    tex.typ().gl(),
                    Some(tex.nat()),
                    0,
                );

                Tx { tex, format }
            })
            .collect();

        (renderbuffer, textures)
    }
}

#[derive(Debug)]
enum CompletionError {
    GlError,
    Undefined,
    IncompleteAttachment,
    IncompleteMissingAttachment,
    IncompleteDrawBuffer,
    IncompleteReadBuffer,
    Unsupported,
    IncompleteMultisample,
    IncompleteLayerTargets,
}

impl CompletionError {
    fn from_error(code: u32) -> Self {
        match code {
            0 => Self::GlError,
            glow::FRAMEBUFFER_UNDEFINED => Self::Undefined,
            glow::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => Self::IncompleteAttachment,
            glow::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => Self::IncompleteMissingAttachment,
            glow::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => Self::IncompleteDrawBuffer,
            glow::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => Self::IncompleteReadBuffer,
            glow::FRAMEBUFFER_UNSUPPORTED => Self::Unsupported,
            glow::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => Self::IncompleteMultisample,
            glow::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => Self::IncompleteLayerTargets,
            _ => panic!("undefined error code"),
        }
    }
}
