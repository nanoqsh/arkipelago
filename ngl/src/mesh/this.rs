use crate::{
    layout::Layout,
    mesh::{
        draw, draw_arrays::DrawArrays, element_buffer::ElementBuffer, usage,
        vertex_array::VertexArray, vertex_buffer::VertexBuffer,
    },
};
use glow::Context;
use std::rc::Rc;

pub struct Indexed<L>(VertexArray<L, (usage::Static, usage::Draw), ElementBuffer<draw::Triangles>>);

impl<L> Indexed<L> {
    pub(crate) fn new(ctx: Rc<Context>, verts: &[L], indxs: &[u32]) -> Self
    where
        L: Layout,
    {
        Self(VertexArray::new(
            ctx,
            |ctx| VertexBuffer::from(ctx, verts),
            |ctx| ElementBuffer::new(ctx, indxs),
        ))
    }

    pub(crate) fn bind(&self) {
        self.0.bind()
    }

    pub(crate) fn draw(&self) {
        self.0.draw()
    }
}

pub struct DynamicTriangleFan<L>(
    VertexArray<L, (usage::Dynamic, usage::Draw), DrawArrays<draw::TriangleFan>>,
);

impl<L> DynamicTriangleFan<L> {
    pub(crate) fn new(ctx: Rc<Context>, len: usize) -> Self
    where
        L: Layout,
    {
        Self(VertexArray::new(
            ctx,
            |ctx| VertexBuffer::empty(ctx, len),
            |_| DrawArrays::new(len),
        ))
    }

    pub(crate) fn update(&self, verts: &[L]) {
        self.0.update(verts)
    }

    pub(crate) fn bind(&self) {
        self.0.bind()
    }

    pub(crate) fn draw(&self) {
        self.0.draw()
    }
}

pub struct DynamicLines<L>(VertexArray<L, (usage::Dynamic, usage::Draw), DrawArrays<draw::Lines>>);

impl<L> DynamicLines<L> {
    pub(crate) fn new(ctx: Rc<Context>, len: usize) -> Self
    where
        L: Layout,
    {
        Self(VertexArray::new(
            ctx,
            |ctx| VertexBuffer::empty(ctx, len),
            |_| DrawArrays::new(len),
        ))
    }

    pub(crate) fn update(&self, verts: &[L]) {
        self.0.update(verts)
    }

    pub(crate) fn bind(&self) {
        self.0.bind()
    }

    pub(crate) fn draw(&self) {
        self.0.draw()
    }
}
