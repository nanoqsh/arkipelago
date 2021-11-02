use crate::mesh::Mesh;
use ngl::{mesh::Indexed, texture, Parameters, Pipe};
use shr::cgm::*;

type Ren = ngl::Render;
pub type Vert = ngl::vertex::Vertex;
pub type Texture = ngl::texture::Texture;

pub struct Render {
    ren: Ren,
    view: Option<Mat4>,
    proj: Option<Mat4>,
}

impl Render {
    /// # Safety
    ///   - Pass correct `load` function.
    ///   - The context must be current and don't changes.
    pub unsafe fn new<F>(load: F) -> Self
    where
        F: FnMut(&str) -> *const (),
    {
        use std::cell::Cell;

        thread_local! {
            static INIT: Cell<bool> = Cell::new(false);
        }

        INIT.with(|initialized| {
            if initialized.get() {
                panic!("Render has already been created once")
            }

            initialized.set(true);
        });

        Self {
            ren: Ren::new(load),
            view: None,
            proj: None,
        }
    }

    pub fn make_texture(&self, data: &[u8], size: UVec2, params: texture::Parameters) -> Texture {
        self.ren.make_texture(data, size, params)
    }

    pub fn make_mesh(&self, mesh: &Mesh<Vert>) -> Indexed<Vert> {
        self.ren.make_indexed_mesh(mesh.verts(), mesh.indxs())
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.ren.resize(size.into())
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = Some(view)
    }

    pub fn set_proj(&mut self, proj: Mat4) {
        self.proj = Some(proj)
    }

    pub fn draw<'a, D>(&mut self, draws: D)
    where
        D: IntoIterator<Item = &'a dyn Pipe>,
    {
        self.ren.draw(
            draws,
            Parameters {
                cl: Vec3::new(0.2, 0., 0.1),
                view: self.view.take().as_ref(),
                proj: self.proj.take().as_ref(),
            },
        )
    }
}
