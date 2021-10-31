use crate::mesh::{
    clear_context::ClearContext,
    draw::{Draw, Mode},
};
use glow::{Context, HasContext, NativeBuffer};
use std::marker::PhantomData;

pub(crate) struct ElementBuffer<M> {
    nat: NativeBuffer,
    len: usize,
    mode: PhantomData<M>,
}

impl<M> ElementBuffer<M> {
    pub fn new(ctx: &Context, indxs: &[u32]) -> Self {
        unsafe {
            let nat = ctx.create_buffer().expect("create buffer");
            ctx.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(nat));

            let len = indxs.len();
            let size = len * std::mem::size_of::<u32>();
            ctx.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                std::slice::from_raw_parts(indxs.as_ptr().cast(), size),
                glow::STATIC_DRAW,
            );

            Self {
                nat,
                len,
                mode: PhantomData,
            }
        }
    }
}

impl<M> ClearContext for ElementBuffer<M> {
    fn clear(&mut self, ctx: &Context) {
        unsafe { ctx.delete_buffer(self.nat) }
    }
}

impl<M> Draw for ElementBuffer<M>
where
    M: Mode,
{
    fn draw(&self, ctx: &Context) {
        unsafe { ctx.draw_elements(M::MODE, self.len as _, glow::UNSIGNED_INT, 0) }
    }
}
