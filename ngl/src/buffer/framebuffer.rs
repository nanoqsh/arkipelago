use crate::buffer::{
    renderbuffer::{self, Renderbuffer},
    texture::{self, Texture},
};
use glow::{Context, HasContext, NativeFramebuffer};
use shr::cgm::*;
use std::rc::Rc;

type Rb = (Renderbuffer, renderbuffer::Format);
type Tx = (Texture, texture::Format);

pub(crate) struct Framebuffer {
    nat: NativeFramebuffer,
    ctx: Rc<Context>,
    renderbuffer: Option<Rb>,
    textures: Vec<Tx>,
}

impl Framebuffer {
    pub fn new(ctx: Rc<Context>, attachments: Attachments, size: UVec2) -> Self {
        unsafe {
            let nat = ctx.create_framebuffer().expect("create framebuffer");
            ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(nat));
            let (renderbuffer, textures) = attachments.attach(Rc::clone(&ctx), size);

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
        if let Some((ren, format)) = &mut self.renderbuffer {
            ren.resize(size, *format)
        }

        for (tex, format) in &mut self.textures {
            tex.resize(&[], size, *format)
        }
    }

    pub fn bind(&self) {
        unsafe { self.ctx.bind_framebuffer(glow::FRAMEBUFFER, Some(self.nat)) }
    }

    pub fn bind_default(&self) {
        unsafe { self.ctx.bind_framebuffer(glow::FRAMEBUFFER, None) }
    }

    pub fn texture(&self, idx: usize) -> Option<&Texture> {
        self.textures.get(idx).map(|(tex, _)| tex)
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

pub(crate) struct Attachments<'a> {
    pub renderbuffer: Option<renderbuffer::Format>,
    pub textures: &'a [texture::Format],
}

impl Attachments<'_> {
    fn attach(self, ctx: Rc<Context>, size: UVec2) -> (Option<Rb>, Vec<Tx>) {
        let renderbuffer = self.renderbuffer.map(|format| unsafe {
            let renderbuffer = Renderbuffer::new(Rc::clone(&ctx), size, format);
            ctx.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                format.gl_attachment(),
                glow::RENDERBUFFER,
                Some(renderbuffer.nat()),
            );
            (renderbuffer, format)
        });

        let textures = self
            .textures
            .iter()
            .enumerate()
            .map(|(n, &format)| unsafe {
                let parameters = texture::Parameters {
                    format,
                    ..texture::Parameters::default()
                };

                let texture = Texture::empty(Rc::clone(&ctx), size, parameters);
                ctx.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    glow::COLOR_ATTACHMENT0 + n as u32,
                    glow::TEXTURE_2D,
                    Some(texture.nat()),
                    0,
                );
                (texture, format)
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
