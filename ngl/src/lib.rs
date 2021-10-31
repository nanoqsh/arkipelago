mod attribute;
mod buffer;
mod debug;
mod declare;
mod layout;
mod line;
pub mod mesh;
mod program;
mod quad;
mod render;
mod shader;
#[allow(dead_code)]
mod shader_set;
mod uniform;
pub mod vertex;

pub mod texture_parameters {
    pub use crate::buffer::texture::{Filter, Format, Parameters, Wrap};
}

pub use crate::{buffer::texture::Texture, render::Render};

pub const GL_VERSION: (u8, u8) = (3, 3);
