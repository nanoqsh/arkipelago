use crate::mesh::{
    clear_context::ClearContext, draw::Draw, usage::Dynamic, vertex_buffer::VertexBuffer,
};
use glow::{Context, HasContext, NativeVertexArray};
use std::rc::Rc;

pub(crate) struct VertexArray<L, U, D: ClearContext> {
    nat: NativeVertexArray,
    ctx: Rc<Context>,
    buf: VertexBuffer<L, U>,
    drw: D,
}

impl<L, U, D: ClearContext> VertexArray<L, U, D> {
    pub fn new<F, G>(ctx: Rc<Context>, make_buf: F, make_drw: G) -> Self
    where
        F: FnOnce(&Context) -> VertexBuffer<L, U>,
        G: FnOnce(&Context) -> D,
    {
        unsafe {
            let nat = ctx.create_vertex_array().expect("create vertex array");
            ctx.bind_vertex_array(Some(nat));

            let buf = make_buf(&ctx);
            let drw = make_drw(&ctx);

            Self { nat, ctx, buf, drw }
        }
    }

    pub fn bind(&self) {
        unsafe { self.ctx.bind_vertex_array(Some(self.nat)) }
    }

    pub fn draw(&self)
    where
        D: Draw,
    {
        self.drw.draw(&self.ctx)
    }
}

impl<L, A, D: ClearContext> VertexArray<L, (Dynamic, A), D> {
    pub fn update(&self, verts: &[L]) {
        self.buf.update(&self.ctx, verts)
    }
}

impl<L, U, D: ClearContext> Drop for VertexArray<L, U, D> {
    fn drop(&mut self) {
        self.buf.clear(&self.ctx);
        self.drw.clear(&self.ctx);
        unsafe { self.ctx.delete_vertex_array(self.nat) }
    }
}
