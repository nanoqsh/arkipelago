use crate::{
    buffer::{
        texture::{self, Texture},
        Frame,
    },
    debug::{debug_gl, Debugger},
    line::Line,
    mesh::Indexed,
    pass::{ColorInner, InterfaceInner, SkinInner, SolidInner},
    quad::Quad,
    shaders::Shaders,
    vertex::Vertex,
    Pipe, Pipeline,
};
use glow::{Context, HasContext};
use shr::{cgm::*, shapes::*};
use std::rc::Rc;

pub struct Render {
    shaders: Shaders,
    frame: Frame,
    pipeline: Pipeline<'static>,
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

        Self {
            shaders: Shaders::new(Rc::clone(&ctx)),
            frame: Frame::new(Rc::clone(&ctx), UVec2::new(1, 1), 0),
            pipeline: Pipeline::default(),
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
        self.frame.resize(size);

        debug_gl!(self.deb);
    }

    pub fn make_texture(&self, data: &[u8], size: UVec2, params: texture::Parameters) -> Texture {
        let tex = Texture::new(Rc::clone(&self.ctx), data, size, params);
        debug_gl!(self.deb);
        tex
    }

    pub fn make_indexed_mesh(&self, verts: &[Vertex], indxs: &[u32]) -> Indexed<Vertex> {
        let mesh = Indexed::new(Rc::clone(&self.ctx), verts, indxs);
        debug_gl!(self.deb);
        mesh
    }

    pub fn draw<'a, D>(&mut self, draws: D, params: Parameters)
    where
        D: IntoIterator<Item = &'a dyn Pipe>,
    {
        let Parameters { cl, view, proj } = params;
        let pipeline = std::mem::take(&mut self.pipeline).filled(draws);

        self.frame.bind();
        unsafe {
            let (r, g, b) = cl.into();
            self.ctx.clear_color(r, g, b, 1.);
            self.ctx.clear_depth_f32(1.);
            self.ctx
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
        debug_gl!(self.deb);

        unsafe {
            self.ctx.enable(glow::DEPTH_TEST);
            self.ctx.depth_func(glow::LESS);
        }
        let inner = SolidInner::new(&self.shaders.solid);
        if let Some(view) = view {
            inner.set_view(view)
        }
        if let Some(proj) = proj {
            inner.set_proj(proj)
        }
        pipeline.draw_solid(inner);
        debug_gl!(self.deb);

        let inner = SkinInner::new(&self.shaders.skin);
        if let Some(view) = view {
            inner.set_view(view)
        }
        if let Some(proj) = proj {
            inner.set_proj(proj)
        }
        pipeline.draw_skin(inner);
        debug_gl!(self.deb);

        unsafe { self.ctx.disable(glow::DEPTH_TEST) }
        let inner = ColorInner::new(&self.shaders.color, &self.line);
        if let Some(view) = view {
            inner.set_view(view)
        }
        if let Some(proj) = proj {
            inner.set_proj(proj)
        }
        pipeline.draw_color(inner);
        debug_gl!(self.deb);

        self.frame.bind_default();
        let frame = self.frame.texture(0).unwrap();
        frame.bind(Shaders::T0);
        self.shaders.post.use_program();
        self.quad.draw(
            Rect::new((-1., -1.), (1., 1.)),
            Rect::new((0., 0.), (1., 1.)),
        );
        debug_gl!(self.deb);

        pipeline.draw_interface(InterfaceInner(()));
        debug_gl!(self.deb);

        self.pipeline = pipeline.cleared();
    }
}

pub struct Parameters<'a> {
    pub cl: Vec3,
    pub view: Option<&'a Mat4>,
    pub proj: Option<&'a Mat4>,
}
