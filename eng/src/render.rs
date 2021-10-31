use crate::{atlas::Atlas, loader::Loader, Ren};
use image::GenericImageView;
use shr::cgm::*;
use std::{cell::RefCell, rc::Rc};

pub struct Render {
    ren: Ren,
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
        }
    }

    pub fn init(self) -> Self {
        let sprites = Rc::new(RefCell::new(Vec::new()));
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

        loader.on_load_sprite({
            let sprites = Rc::clone(&sprites);
            move |name, sprite| {
                let (width, height) = sprite.dimensions();
                println!("Loaded sprite {} (size: ({}, {}))", name, width, height);
                sprites.borrow_mut().push(sprite);
            }
        });

        let mesh = loader.load_mesh("cube").unwrap();
        let _ = self.ren.make_indexed_mesh(mesh.verts(), mesh.indxs());

        loader.load_texture("tiles/stone", &self.ren).unwrap();
        loader.load_sprite("tiles/stone").unwrap();
        loader.load_sprite("tiles/dirt").unwrap();

        let atlas = Atlas::new(sprites.borrow().iter().map(Rc::as_ref)).unwrap();
        let _ = atlas.map();
        let _ = atlas.addition_fn();
        let _ = atlas.multiplier();

        self
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.ren.resize(size.into())
    }

    pub fn draw(&mut self) {
        self.ren.start_frame();
        self.ren.clear(Vec3::new(0.1, 0.0, 0.2));
        self.ren.finish_frame();
    }
}
