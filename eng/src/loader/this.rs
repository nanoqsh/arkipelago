use crate::{
    loader::{
        cached::Cached,
        load::{MeshLoad, SpriteLoad, TextureLoad},
        re::*,
    },
    mesh::Mesh,
    Ren, Texture, Vert,
};
use image::DynamicImage;
use std::{path::PathBuf, rc::Rc};

pub(crate) struct Loader {
    path: PathBuf,
    buf: String,
    meshes: Cached<Mesh<Vert>>,
    textures: Cached<Texture>,
    sprites: Cached<DynamicImage>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            path: PathBuf::with_capacity(64),
            buf: String::with_capacity(64),
            meshes: Cached::with_capacity(8),
            textures: Cached::with_capacity(8),
            sprites: Cached::with_capacity(8),
        }
    }

    pub fn load_mesh<S>(&mut self, name: S) -> Result<Rc<Mesh<Vert>>, Error>
    where
        S: Into<String>,
    {
        self.meshes.load(name, |name| {
            read(&mut self.path, name, MeshLoad, Json::new(&mut self.buf))
        })
    }

    pub fn load_texture<S>(&mut self, name: S, ren: &Ren) -> Result<Rc<Texture>, Error>
    where
        S: Into<String>,
    {
        self.textures.load(name, |name| {
            read(&mut self.path, name, TextureLoad { ren }, Png)
        })
    }

    pub fn load_sprite<S>(&mut self, name: S) -> Result<Rc<DynamicImage>, Error>
    where
        S: Into<String>,
    {
        self.sprites
            .load(name, |name| read(&mut self.path, name, SpriteLoad, Png))
    }

    pub fn on_load_mesh<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<Mesh<Vert>>) + 'static,
    {
        self.meshes.on_load(Box::new(event))
    }

    pub fn on_load_texture<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<Texture>) + 'static,
    {
        self.textures.on_load(Box::new(event))
    }

    pub fn on_load_sprite<F>(&mut self, event: F)
    where
        F: FnMut(&str, Rc<DynamicImage>) + 'static,
    {
        self.sprites.on_load(Box::new(event))
    }
}

fn read<'a, L>(
    path: &mut PathBuf,
    name: &str,
    load: L,
    format: L::Format,
) -> Result<L::Asset, Error>
where
    L: Load<'a>,
{
    path.clear();
    path.push(ASSETS_PATH);
    path.push(L::PATH);
    path.push(name);
    path.set_extension(<L::Format as Format>::EXT);

    println!("[ DEBUG ] Read: {:?}", path);

    let raw = format.read(path)?;
    load.load(raw)
}
