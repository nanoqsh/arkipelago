use crate::{
    layout::{Field, Layout},
    mesh::{
        clear_context::ClearContext,
        usage::{Dynamic, Usage},
    },
};
use glow::{Context, HasContext, NativeBuffer};
use std::marker::PhantomData;

pub(crate) struct VertexBuffer<L, U> {
    nat: NativeBuffer,
    layout: PhantomData<L>,
    usage: PhantomData<U>,
}

impl<L: Layout, U: Usage> VertexBuffer<L, U> {
    pub fn empty(ctx: &Context, len: usize) -> Self {
        unsafe { Self::new(ctx, std::ptr::null(), len) }
    }

    pub fn from(ctx: &Context, verts: &[L]) -> Self {
        unsafe { Self::new(ctx, verts.as_ptr(), verts.len()) }
    }

    unsafe fn new(ctx: &Context, ptr: *const L, len: usize) -> Self {
        let nat = ctx.create_buffer().expect("create buffer");
        ctx.bind_buffer(glow::ARRAY_BUFFER, Some(nat));

        let size = len * std::mem::size_of::<L>();
        if ptr.is_null() {
            ctx.buffer_data_size(glow::ARRAY_BUFFER, size as _, U::USAGE)
        } else {
            ctx.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                std::slice::from_raw_parts(ptr.cast(), size),
                U::USAGE,
            )
        }

        let mut fields = Vec::new();
        L::layout(&mut fields);

        assert!(fields.len() <= glow::MAX_VERTEX_ATTRIBS as _);

        for (idx, field) in fields.into_iter().enumerate() {
            Self::vertex_attrib_pointer(ctx, idx as _, field, std::mem::size_of::<L>() as _);
            ctx.enable_vertex_attrib_array(idx as _);
        }

        Self {
            nat,
            layout: PhantomData,
            usage: PhantomData,
        }
    }

    unsafe fn vertex_attrib_pointer(ctx: &Context, idx: u32, field: Field, stride: i32) {
        match field.element_type {
            glow::BYTE
            | glow::UNSIGNED_BYTE
            | glow::SHORT
            | glow::UNSIGNED_SHORT
            | glow::INT
            | glow::UNSIGNED_INT => ctx.vertex_attrib_pointer_i32(
                idx,
                field.components as _,
                field.element_type,
                stride,
                field.offset as _,
            ),
            glow::HALF_FLOAT
            | glow::FLOAT
            | glow::DOUBLE
            | glow::FIXED
            | glow::INT_2_10_10_10_REV
            | glow::UNSIGNED_INT_2_10_10_10_REV
            | glow::UNSIGNED_INT_10F_11F_11F_REV => ctx.vertex_attrib_pointer_f32(
                idx,
                field.components as _,
                field.element_type,
                false,
                stride,
                field.offset as _,
            ),
            _ => panic!("unsupported data type"),
        }
    }
}

impl<L, A> VertexBuffer<L, (Dynamic, A)> {
    pub fn update(&self, ctx: &Context, verts: &[L]) {
        unsafe {
            ctx.bind_buffer(glow::ARRAY_BUFFER, Some(self.nat));
            ctx.buffer_sub_data_u8_slice(
                glow::ARRAY_BUFFER,
                0,
                std::slice::from_raw_parts(
                    verts.as_ptr().cast(),
                    verts.len() * std::mem::size_of::<L>(),
                ),
            )
        }
    }
}

impl<L, U> ClearContext for VertexBuffer<L, U> {
    fn clear(&mut self, ctx: &Context) {
        unsafe { ctx.delete_buffer(self.nat) }
    }
}
