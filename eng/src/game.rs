use crate::{atlas::Atlas, camera::TpCamera, land::Factory, loader::Loader, Render, Texture, Vert};
use image::{DynamicImage, GenericImageView};
use ngl::{
    mesh::Indexed,
    pass::{Pass, Solid, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;
use std::{collections::HashMap, rc::Rc};

struct Data {
    pub mesh: Indexed<Vert>,
    pub tex: Rc<Texture>,
}

impl Draw<Solid> for Data {
    fn draw<'a>(&self, pass: Pass<'a, Solid>)
    where
        Solid: Stage<'a>,
    {
        pass.set_model(&Mat4::identity());
        pass.set_texture(&self.tex);
        pass.draw_indexed_mesh(&self.mesh);
    }
}

impl Pipe for Data {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        pipeline.push_solid(self);
    }
}

#[derive(Debug)]
pub enum Control {
    Look(f32, f32),
    Scroll(f32, f32),
    Forward,
    Back,
    Left,
    Right,
}

pub struct Game {
    data: Data,
    cam: TpCamera,
}

impl Game {
    pub fn new(ren: &Render) -> Self {
        let mut sprite_names = HashMap::new();
        let mut sprites = Vec::new();
        let mut loader = Loader::new(ren);

        loader.on_load_sprite(|name, sprite: Rc<DynamicImage>| {
            let (width, height) = sprite.dimensions();
            println!("Loaded sprite {} (size: ({}, {}))", name, width, height);

            if sprites.is_empty() {
                debug_assert_eq!(name, "tiles/default")
            }

            assert!(sprite_names
                .insert(name.to_string(), sprites.len() as u32)
                .is_none());
            sprites.push(sprite);
        });
        loader.on_load_texture(|_, _| ());
        loader.on_load_mesh(|_, _| ());

        loader.load_sprite("tiles/default").unwrap();
        loader.load_sprite("tiles/stone").unwrap();
        loader.load_sprite("tiles/dirt").unwrap();

        let mesh = loader.load_mesh("cube").unwrap();
        let cube = ren.make_mesh(&mesh);
        let stone = loader.load_texture("tiles/stone").unwrap();

        let grass = loader.load_variant("grass").unwrap();
        let samples = loader.take_samples();
        drop(loader);

        let atlas = Atlas::new(sprites.iter().map(Rc::as_ref)).unwrap();
        let (_, mapper) = atlas.map();

        let _ = Factory::new(mapper);

        Self {
            data: Data {
                mesh: cube,
                tex: stone,
            },
            cam: TpCamera::new(1., Pnt3::origin()),
        }
    }

    pub fn draw(&mut self, ren: &mut Render, _: f32) {
        ren.set_proj(self.cam.proj(1.));
        ren.set_view(self.cam.view());
        ren.draw([&self.data as &dyn Pipe])
    }

    pub fn input(&mut self, control: Control) {
        const SENSITIVITY: f32 = 0.01;

        match control {
            Control::Look(x, y) => self.cam.rotate(Vec2::new(x, y) * SENSITIVITY),
            Control::Scroll(_, y) => self.cam.move_to(y),
            _ => {}
        }
    }
}
