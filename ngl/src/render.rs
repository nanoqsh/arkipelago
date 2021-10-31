use crate::{
    buffer::{
        renderbuffer,
        texture::{self, Parameters, Texture},
        Attachments, Framebuffer,
    },
    debug::{debug_gl, Debugger},
    line::Line,
    mesh::Indexed,
    quad::Quad,
    shader_set::ShaderSet,
    vertex::Vertex,
};
use glow::{Context, HasContext};
use shr::{cgm::*, shapes::*};
use std::rc::Rc;

pub struct Render {
    shader: ShaderSet,
    framebuffer: Framebuffer,
    line: Line,
    quad: Quad,
    deb: Debugger,
    ctx: Rc<Context>,
}

impl Render {
    /// # Safety
    ///   - Pass correct `load` function.
    ///   - The context must be current and don't changes.
    ///   - Don't create more than one object.
    pub unsafe fn new<F>(mut load: F) -> Self
    where
        F: FnMut(&str) -> *const (),
    {
        let ctx = Rc::new(Context::from_loader_function(|s| load(s).cast()));

        let deb = Debugger::new(Rc::clone(&ctx));
        debug_gl!(deb);

        let _framebuffer = {
            Framebuffer::new(
                Rc::clone(&ctx),
                Attachments {
                    renderbuffer: Some(renderbuffer::Format::Depth24),
                    textures: &[texture::Format::Rgb],
                },
                UVec2::new(1, 1),
            )
        };
        debug_gl!(deb);

        let _quad = Quad::new(Rc::clone(&ctx));
        debug_gl!(deb);

        Self {
            shader: ShaderSet::new(Rc::clone(&ctx)),
            framebuffer: {
                Framebuffer::new(
                    Rc::clone(&ctx),
                    Attachments {
                        renderbuffer: Some(renderbuffer::Format::Depth24),
                        textures: &[texture::Format::Rgb],
                    },
                    UVec2::new(1, 1),
                )
            },
            line: Line::new(Rc::clone(&ctx)),
            quad: Quad::new(Rc::clone(&ctx)),
            deb: {
                let deb = Debugger::new(Rc::clone(&ctx));
                debug_gl!(deb);
                deb
            },
            ctx,
        }
    }

    pub fn resize(&mut self, size: UVec2) {
        const MIN_SIZE: u32 = 2;
        const MAX_SIZE: u32 = i32::MAX as u32;

        let (width, height) = match size.into() {
            (width @ MIN_SIZE..=MAX_SIZE, height @ MIN_SIZE..=MAX_SIZE) => (width, height),
            _ => return,
        };

        unsafe { self.ctx.viewport(0, 0, width as _, height as _) }
        self.framebuffer.resize(size);

        debug_gl!(self.deb);
    }

    pub fn make_texture(&self, data: &[u8], size: UVec2, params: Parameters) -> Texture {
        let tex = Texture::new(Rc::clone(&self.ctx), data, size, params);
        debug_gl!(self.deb);
        tex
    }

    pub fn bind_texture(&self, tex: &Texture) {
        self.shader.bind_texture(tex);
        debug_gl!(self.deb);
    }

    pub fn make_indexed_mesh(&self, verts: &[Vertex], indxs: &[u32]) -> Indexed<Vertex> {
        let mesh = Indexed::new(Rc::clone(&self.ctx), verts, indxs);
        debug_gl!(self.deb);
        mesh
    }

    pub fn draw_indexed_mesh(&mut self, mesh: &Indexed<Vertex>) {
        self.shader.use_def();
        mesh.bind();
        mesh.draw();
        debug_gl!(self.deb);
    }

    pub fn draw_line(&mut self, a: Vec3, b: Vec3, cl: Vec3) {
        self.shader.use_col();

        unsafe {
            self.ctx.disable(glow::DEPTH_TEST);
            self.line.draw(a, b, cl);
            self.ctx.enable(glow::DEPTH_TEST);
        }

        debug_gl!(self.deb);
    }

    pub fn clear(&self, cl: Vec3) {
        unsafe {
            let (r, g, b) = cl.into();
            self.ctx.clear_color(r, g, b, 1.);
            self.ctx.clear(glow::COLOR_BUFFER_BIT);
        }

        debug_gl!(self.deb);
    }

    pub fn start_frame(&mut self) {
        self.framebuffer.bind();
        debug_gl!(self.deb);
    }

    pub fn finish_frame(&mut self) {
        self.framebuffer.bind_default();
        let frame = self.framebuffer.texture(0).unwrap();
        self.shader.bind_texture(frame);

        unsafe { self.ctx.disable(glow::DEPTH_TEST) }
        self.shader.use_post();
        self.quad.draw(
            Rect::new((-1., -1.), (1., 1.)),
            Rect::new((0., 0.), (1., 1.)),
        );

        debug_gl!(self.deb);
    }
}
