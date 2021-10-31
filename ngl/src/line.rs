use crate::{mesh::DynamicLines, vertex::ColorVertex};
use glow::Context;
use shr::cgm::*;
use std::rc::Rc;

pub(crate) struct Line(DynamicLines<ColorVertex>);

impl Line {
    pub fn new(ctx: Rc<Context>) -> Self {
        Self(DynamicLines::new(ctx, 2))
    }

    pub fn draw(&self, a: Vec3, b: Vec3, cl: Vec3) {
        let verts = [ColorVertex { co: a, cl }, ColorVertex { co: b, cl }];

        self.0.bind();
        self.0.update(&verts);
        self.0.draw();
    }
}
