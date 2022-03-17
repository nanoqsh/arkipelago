use crate::buffer::{
    framebuffer::{Attachments, Framebuffer},
    renderbuffer,
    texture::{self, Texture},
};
use glow::Context;
use shr::cgm::*;
use std::rc::Rc;

enum Buffers {
    #[allow(dead_code)]
    Single(Framebuffer),
    Double {
        multisample: Framebuffer,
        intermediate: Framebuffer,
    },
}

pub(crate) struct Frame {
    buffers: Buffers,
    size: UVec2,
    factor: u32,
}

impl Frame {
    pub fn new(ctx: Rc<Context>, size: UVec2, factor: u32) -> Self {
        assert!(factor > 0);

        let attachments = Attachments {
            renderbuffer: Some(renderbuffer::Format::Depth24),
            textures: &[texture::Format::Rgb],
        };

        let buffers = Buffers::Double {
            multisample: Framebuffer::new(Rc::clone(&ctx), attachments, size * factor),
            intermediate: Framebuffer::new(ctx, attachments, size),
        };

        Self {
            buffers,
            size,
            factor,
        }
    }

    pub fn resize(&mut self, size: UVec2) {
        match &mut self.buffers {
            Buffers::Single(buffer) => buffer.resize(size),
            Buffers::Double {
                multisample,
                intermediate,
            } => {
                multisample.resize(size * self.factor);
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
                multisample.blit_to(intermediate, self.size, self.factor);
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
