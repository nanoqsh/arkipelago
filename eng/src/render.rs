use crate::{atlas::Atlas, loader::Loader, Ren, Vert};
use image::GenericImageView;
use ngl::{mesh::Indexed, texture::Texture, Parameters, Pipe};
use shr::cgm::*;
use std::{cell::RefCell, rc::Rc};

pub struct Data {
    pub mesh: Indexed<Vert>,
    pub tex: Rc<Texture>,
}

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

    pub fn init(self) -> (Self, Data) {
        let mut loader = Loader::new();
        loader.on_load_mesh(|name, mesh| {
            println!(
                "Loaded mesh {} (indxs: {}, verts: {}, slots: {})",
                name,
                mesh.indxs().len(),
                mesh.verts().len(),
                mesh.slots().len(),
            )
        });

        loader.on_load_texture(|name, tex| {
            let (width, height) = tex.size().into();
            println!("Loaded texture {} (size: ({}, {}))", name, width, height)
        });

        let sprites = Rc::new(RefCell::new(Vec::new()));
        loader.on_load_sprite({
            let sprites = Rc::clone(&sprites);
            move |name, sprite| {
                let (width, height) = sprite.dimensions();
                println!("Loaded sprite {} (size: ({}, {}))", name, width, height);
                sprites.borrow_mut().push(sprite);
            }
        });

        let mesh = loader.load_mesh("cube").unwrap();
        let cube = self.ren.make_indexed_mesh(mesh.verts(), mesh.indxs());
        let stone = loader.load_texture("tiles/stone", &self.ren).unwrap();

        loader.load_sprite("tiles/stone").unwrap();
        loader.load_sprite("tiles/dirt").unwrap();
        let atlas = Atlas::new(sprites.borrow().iter().map(Rc::as_ref)).unwrap();
        let _ = atlas.map();
        let _ = atlas.addition_fn();
        let _ = atlas.multiplier();

        (
            self,
            Data {
                mesh: cube,
                tex: stone,
            },
        )
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
