use glow::Context;

pub(crate) trait Draw {
    fn draw(&self, ctx: &Context);
}

pub(crate) trait Mode {
    const MODE: u32;
}

pub(crate) struct Points;

impl Mode for Points {
    const MODE: u32 = glow::POINTS;
}

pub(crate) struct LineStrip;

impl Mode for LineStrip {
    const MODE: u32 = glow::LINE_STRIP;
}

pub(crate) struct LineLoop;

impl Mode for LineLoop {
    const MODE: u32 = glow::LINE_LOOP;
}

pub(crate) struct Lines;

impl Mode for Lines {
    const MODE: u32 = glow::LINES;
}

pub(crate) struct TriangleStrip;

impl Mode for TriangleStrip {
    const MODE: u32 = glow::TRIANGLE_STRIP;
}

pub(crate) struct TriangleFan;

impl Mode for TriangleFan {
    const MODE: u32 = glow::TRIANGLE_FAN;
}

pub(crate) struct Triangles;

impl Mode for Triangles {
    const MODE: u32 = glow::TRIANGLES;
}
