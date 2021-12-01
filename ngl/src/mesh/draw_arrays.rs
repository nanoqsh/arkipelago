use crate::mesh::{
    clear_context::ClearContext,
    draw::{Draw, Mode},
};
use glow::{Context, HasContext};
use std::marker::PhantomData;

pub(crate) struct DrawArrays<M> {
    len: usize,
    mode: PhantomData<M>,
}

impl<M> DrawArrays<M> {
    pub fn new(len: usize) -> Self {
        Self {
            len,
            mode: PhantomData,
        }
    }
}

impl<M> ClearContext for DrawArrays<M> {
    fn clear(&mut self, _: &Context) {}
}

impl<M: Mode> Draw for DrawArrays<M> {
    fn draw(&self, ctx: &Context) {
        unsafe { ctx.draw_arrays(M::MODE, 0, self.len as _) }
    }
}
