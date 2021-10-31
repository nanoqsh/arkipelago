use crate::{mesh::DynamicTriangleFan, vertex::PostVertex};
use glow::Context;
use shr::shapes::*;
use std::rc::Rc;

pub(crate) struct Quad(DynamicTriangleFan<PostVertex>);

impl Quad {
    pub fn new(ctx: Rc<Context>) -> Self {
        Self(DynamicTriangleFan::new(ctx, 4))
    }

    pub fn draw(&self, co: Rect, st: Rect) {
        let co = co.rect_points();
        let st = st.rect_points();

        let verts = [
            PostVertex {
                co: co[0],
                st: st[0],
            },
            PostVertex {
                co: co[1],
                st: st[1],
            },
            PostVertex {
                co: co[2],
                st: st[2],
            },
            PostVertex {
                co: co[3],
                st: st[3],
            },
        ];

        self.0.bind();
        self.0.update(&verts);
        self.0.draw();
    }
}
