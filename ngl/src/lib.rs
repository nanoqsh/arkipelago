mod attribute;
mod buffer;
mod debug;
mod declare;
mod draw;
mod layout;
mod line;
pub mod mesh;
pub mod pass;
#[allow(dead_code)]
mod program;
mod quad;
mod render;
mod shader;
mod shaders;
mod uniform;
pub mod vertex;

pub mod texture {
    pub use crate::buffer::texture::{Filter, Format, Parameters, Texture, Wrap};
}

pub use crate::{
    draw::{Draw, Pipe, Pipeline},
    render::{Parameters, Render},
};

pub const GL_VERSION: (u8, u8) = (3, 3);
