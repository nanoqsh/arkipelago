use crate::{
    loader::{
        load::{MeshLoad, Sample, SpriteLoad, TextureLoad, ToVariant, VariantLoad},
        re::*,
        reader::Reader,
    },
    Mesh, Render, Texture,
};
use image::DynamicImage;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

pub(crate) struct Loader<'a> {
    ren: &'a Render,
    textures: Reader<'a, Texture>,
    sprites: Reader<'a, DynamicImage>,
    meshes: Reader<'a, Mesh, String>,
    samples: Reader<'a, Sample, String>,
    variants: Reader<'a, ToVariant, String>,
}

impl<'a> Loader<'a> {
    pub fn new(ren: &'a Render) -> Self {
        let buf = Rc::new(RefCell::new(PathBuf::with_capacity(64)));

        Self {
            ren,
            textures: Reader::with_capacity((), Rc::clone(&buf), 8),
            sprites: Reader::with_capacity((), Rc::clone(&buf), 8),
            meshes: Reader::with_capacity(String::with_capacity(64), Rc::clone(&buf), 8),
            samples: Reader::with_capacity(String::with_capacity(64), Rc::clone(&buf), 8),
            variants: Reader::with_capacity(String::with_capacity(64), Rc::clone(&buf), 8),
        }
    }

    pub fn load_texture(&mut self, name: &str) -> Result<Rc<Texture>, Error> {
        self.textures.read_png(name, TextureLoad { ren: self.ren })
    }

    pub fn load_sprite(&mut self, name: &str) -> Result<Rc<DynamicImage>, Error> {
        self.sprites.read_png(name, SpriteLoad)
    }

    pub fn load_mesh(&mut self, name: &str) -> Result<Rc<Mesh>, Error> {
        self.meshes.read_json(name, MeshLoad)
    }

    pub fn load_variant(&mut self, name: &str) -> Result<Rc<ToVariant>, Error> {
        self.variants.read_json(
            name,
            VariantLoad {
                sprites: &mut self.sprites,
                meshes: &mut self.meshes,
                samples: &mut self.samples,
            },
        )
    }

    pub fn on_load_texture<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<Texture>) + 'a,
    {
        self.textures.on_load(Box::new(event))
    }

    pub fn on_load_sprite<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<DynamicImage>) + 'a,
    {
        self.sprites.on_load(Box::new(event))
    }

    pub fn on_load_mesh<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<Mesh>) + 'a,
    {
        self.meshes.on_load(Box::new(event))
    }
}
