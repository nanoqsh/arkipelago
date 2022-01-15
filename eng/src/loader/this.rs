use crate::{
    land::polygon::Polygons,
    loader::{
        load::{
            Cached, EventLoad, Load, MeshLoad, SampleLoad, SpriteLoad, TextureLoad, ToVariant,
            VariantLoad,
        },
        read::{ReadImage, ReadJson},
        Error,
    },
    Mesh, Render, Texture,
};
use image::DynamicImage;
use std::{cell::RefCell, rc::Rc};

pub(crate) struct Loader<'a> {
    textures: Cached<EventLoad<'a, TextureLoad<'a>>>,
    sprites: Rc<RefCell<Cached<EventLoad<'a, SpriteLoad>>>>,
    meshes: Rc<RefCell<Cached<EventLoad<'a, MeshLoad>>>>,
    samples: Rc<RefCell<Cached<EventLoad<'a, SampleLoad<'a>>>>>,
    variants: Cached<EventLoad<'a, VariantLoad<'a>>>,
}

impl<'a> Loader<'a> {
    pub fn new(ren: &'a Render) -> Self {
        let sprites = Rc::new(RefCell::new(Cached::new(EventLoad::new(SpriteLoad {
            read: ReadImage::new("textures/tiles"),
        }))));

        let meshes = Rc::new(RefCell::new(Cached::new(EventLoad::new(MeshLoad {
            read: ReadJson::new("meshes"),
        }))));

        let samples = Rc::new(RefCell::new(Cached::new(EventLoad::new(SampleLoad {
            read: ReadJson::new("samples"),
            meshes: Rc::clone(&meshes),
            polygons: Polygons::with_capacity(16),
        }))));

        Self {
            textures: Cached::new(EventLoad::new(TextureLoad {
                read: ReadImage::new("textures"),
                ren,
            })),
            sprites: Rc::clone(&sprites),
            meshes,
            samples: Rc::clone(&samples),
            variants: Cached::new(EventLoad::new(VariantLoad {
                read: ReadJson::new("variants"),
                sprites,
                samples,
            })),
        }
    }

    pub fn load_texture(&mut self, name: &str) -> Result<Rc<Texture>, Error> {
        self.textures.load(name)
    }

    pub fn load_sprite(&mut self, name: &str) -> Result<Rc<DynamicImage>, Error> {
        self.sprites.borrow_mut().load(name)
    }

    pub fn load_mesh(&mut self, name: &str) -> Result<Rc<Mesh>, Error> {
        self.meshes.borrow_mut().load(name)
    }

    pub fn load_variant(&mut self, name: &str) -> Result<Rc<ToVariant>, Error> {
        self.variants.load(name)
    }

    pub fn on_load_texture<F>(&mut self, event: F)
    where
        F: FnMut(&str, &Texture) + 'a,
    {
        self.textures.set_event(Box::new(event))
    }

    pub fn on_load_sprite<F>(&mut self, event: F)
    where
        F: FnMut(&str, &DynamicImage) + 'a,
    {
        self.sprites.borrow_mut().set_event(Box::new(event))
    }

    pub fn on_load_mesh<F>(&mut self, event: F)
    where
        F: FnMut(&str, &Mesh) + 'a,
    {
        self.meshes.borrow_mut().set_event(Box::new(event))
    }

    pub fn take_polygons(&mut self) -> Polygons {
        let polygons = &mut self.samples.borrow_mut().polygons;
        polygons.shrink_to_fit();
        std::mem::take(polygons)
    }
}
