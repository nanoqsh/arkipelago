mod atlas;
#[allow(dead_code)]
mod camera;
mod game;
mod land;
mod loader;
#[allow(dead_code)]
mod mesh;
mod render;

pub use self::{
    game::{Control, Game},
    render::Render,
};

type Vert = ngl::vertex::Vertex;
type Texture = ngl::texture::Texture;
type Mesh = self::mesh::Mesh<Vert, str>;

pub use ngl::GL_VERSION;
