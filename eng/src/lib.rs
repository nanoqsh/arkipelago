mod atlas;
#[allow(dead_code)]
mod camera;
mod game;
mod loader;
#[allow(dead_code)]
mod mesh;
mod render;

pub use self::{
    game::{Control, Game},
    render::Render,
};
pub use ngl::GL_VERSION;

pub type Ren = ngl::Render;
pub type Vert = ngl::vertex::Vertex;
pub type Texture = ngl::Texture;
