pub(crate) trait Usage {
    const USAGE: u32;
}

pub(crate) struct Draw;
pub(crate) struct Read;
pub(crate) struct Copy;

pub(crate) struct Stream;

impl Usage for (Stream, Draw) {
    const USAGE: u32 = glow::STREAM_DRAW;
}

impl Usage for (Stream, Read) {
    const USAGE: u32 = glow::STREAM_READ;
}

impl Usage for (Stream, Copy) {
    const USAGE: u32 = glow::STREAM_COPY;
}

pub(crate) struct Static;

impl Usage for (Static, Draw) {
    const USAGE: u32 = glow::STATIC_DRAW;
}

impl Usage for (Static, Read) {
    const USAGE: u32 = glow::STATIC_READ;
}

impl Usage for (Static, Copy) {
    const USAGE: u32 = glow::STATIC_COPY;
}

pub(crate) struct Dynamic;

impl Usage for (Dynamic, Draw) {
    const USAGE: u32 = glow::DYNAMIC_DRAW;
}

impl Usage for (Dynamic, Read) {
    const USAGE: u32 = glow::DYNAMIC_READ;
}

impl Usage for (Dynamic, Copy) {
    const USAGE: u32 = glow::DYNAMIC_COPY;
}
