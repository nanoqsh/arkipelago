use crate::buffer::{
    framebuffer::{Attachments, Framebuffer},
    renderbuffer,
    texture::{self, Texture},
};
use glow::Context;
use shr::cgm::*;
use std::rc::Rc;

enum Buffers {
    Single(Framebuffer),
    Double {
        multisample: Framebuffer,
        intermediate: Framebuffer,
    },
}

pub(crate) struct Frame {
    buffers: Buffers,
    size: UVec2,
}

impl Frame {
    pub fn new(ctx: Rc<Context>, size: UVec2, samples: u8) -> Self {
        let attachments = Attachments {
            renderbuffer: Some(renderbuffer::Format::Depth24),
            textures: &[texture::Format::Rgb],
        };

        let buffers = match samples {
            0 => Buffers::Single(Framebuffer::new(ctx, attachments, size, samples)),
            _ => Buffers::Double {
                multisample: Framebuffer::new(Rc::clone(&ctx), attachments, size, samples),
                intermediate: Framebuffer::new(ctx, attachments, size, 0),
            },
        };

        Self { buffers, size }
    }

    pub fn resize(&mut self, size: UVec2) {
        match &mut self.buffers {
            Buffers::Single(buffer) => buffer.resize(size),
            Buffers::Double {
                multisample,
                intermediate,
            } => {
                multisample.resize(size);
                intermediate.resize(size);
            }
        }
        self.size = size;
    }

    pub fn bind(&self) {
        match &self.buffers {
            Buffers::Single(buffer) => buffer.bind(),
            Buffers::Double { multisample, .. } => multisample.bind(),
        }
    }

    pub fn bind_default(&self) {
        match &self.buffers {
            Buffers::Single(buffer) => buffer.bind_default(),
            Buffers::Double {
                multisample,
                intermediate,
            } => {
                multisample.blit_to(intermediate, self.size);
                multisample.bind_default();
            }
        }
    }

    pub fn texture(&self, idx: usize) -> Option<&Texture> {
        match &self.buffers {
            Buffers::Single(buffer) => buffer.texture(idx),
            Buffers::Double { intermediate, .. } => intermediate.texture(idx),
        }
    }
}
