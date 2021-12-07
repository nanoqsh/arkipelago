use crate::{
    atlas::Atlas,
    camera::TpCamera,
    land::{variant::VariantSet, ClusterView, Factory},
    loader::Loader,
    Render, Texture, Vert,
};
use core::prelude::*;
use image::DynamicImage;
use ngl::{
    mesh::Indexed,
    pass::{Pass, Solid, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;
use std::{collections::HashMap, rc::Rc};

struct Data {
    pub mesh: Indexed<Vert>,
    pub map: Texture,
}

impl Draw<Solid> for Data {
    fn draw<'a>(&self, pass: Pass<'a, Solid>)
    where
        Solid: Stage<'a>,
    {
        pass.set_model(&Mat4::identity());
        pass.set_texture(&self.map);
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
    aspect: f32,
}

impl Game {
    pub fn new(ren: &Render) -> Self {
        let tiles = TileList::new();
        let mut to_variants = Vec::new();

        let mut sprite_names = HashMap::new();
        let mut sprites = Vec::new();

        let mut polygons = {
            let mut loader = Loader::new(ren);
            loader.on_load_sprite(|name, sprite: Rc<DynamicImage>| {
                let (_, name) = name.split_once('/').unwrap();
                if sprites.is_empty() {
                    assert_eq!(name, "default")
                }

                assert!(sprite_names
                    .insert(name.to_string(), sprites.len() as u32)
                    .is_none());
                sprites.push(sprite);
            });
            loader.load_sprite("tiles/default").unwrap();

            for info in tiles.iter() {
                for (name, variant) in info.variants() {
                    let to_variant = loader.load_variant(name).unwrap();
                    to_variants.push(((info.index(), *variant), to_variant));
                }
            }

            loader.take_polygons()
        };

        let atlas = Atlas::new(sprites.iter().map(Rc::as_ref)).unwrap();
        let (map, mapper) = atlas.map();
        let map = ren.make_texture(&map);
        let mut factory = Factory::new(mapper);

        let mut variant_set = VariantSet::new();
        for (key, to_variant) in to_variants {
            let variant = to_variant
                .to_variant(&mut factory, &mut polygons, |sprite| {
                    let idx = match sprite {
                        None => 0,
                        Some(name) => match sprite_names.get(name) {
                            None => panic!("sprite {} not found", name),
                            Some(&idx) => idx,
                        },
                    };
                    mapper.addition(idx) * mapper.multiplier()
                })
                .unwrap();

            variant_set.add(key, variant);
        }

        let mut view = ClusterView::new(variant_set, polygons);
        for (name, (x, y, z), variant) in [
            ("dirt", (0, 0, 0), 0),
            ("grass", (0, 1, 0), 0),
            ("dirt", (1, 0, 0), 0),
            ("grass", (1, 1, 0), 1),
            ("dirt", (2, 0, 0), 2),
            ("dirt", (4, 0, 0), 4),
            ("dirt", (4, 0, 1), 4),
            ("dirt", (4, 0, 2), 4),
            ("stone", (0, 0, 1), 0),
            ("rocks", (0, 2, 1), 0),
            ("stone", (1, 0, 1), 0),
            ("steps", (0, 0, 2), 0),
            ("steps", (1, 0, 2), 1),
            ("steps", (2, 0, 2), 2),
            ("steps", (3, 0, 2), 3),
            ("dirt", (0, 0, 3), 0),
            ("dirt", (1, 0, 3), 0),
            ("dirt", (0, 0, 4), 0),
            ("dirt", (1, 0, 4), 0),
            ("grass", (0, 1, 3), 0),
            ("grass", (0, 1, 4), 1),
            ("rocks", (0, 1, 5), 0),
            ("dirt", (0, 0, 5), 0),
            ("dirt", (1, 0, 5), 0),
            ("dirt", (2, 0, 5), 0),
            ("dirt", (1, 1, 5), 0),
            ("dirt", (2, 1, 5), 0),
            ("dirt", (2, 2, 5), 0),
            ("grass", (2, 3, 5), 0),
            ("dirt", (0, 0, 6), 1),
            ("dirt", (2, 0, 6), 1),
            ("bricks", (2, 0, 3), 0),
            ("bricks", (3, 0, 3), 0),
            ("bricks", (4, 0, 3), 0),
            ("box", (3, 0, 4), 0),
            ("box", (3, 2, 4), 0),
            ("box", (3, 1, 5), 0),
            ("box", (3, 3, 5), 0),
            ("box", (3, 0, 6), 0),
            ("box", (3, 4, 6), 0),
            ("grass", (5, 1, 0), 0),
            ("grass", (5, 1, 3), 1),
            ("dirt", (5, 0, 0), 0),
            ("dirt", (5, 0, 1), 0),
            ("dirt", (5, 0, 2), 0),
            ("dirt", (5, 0, 3), 0),
            ("dirt", (6, 0, 0), 2),
            ("dirt", (6, 0, 1), 2),
            ("dirt", (6, 0, 2), 2),
            ("dirt", (6, 0, 3), 2),
            ("stone", (5, 0, 5), 3),
            ("stone", (5, 0, 6), 1),
            ("stone", (6, 0, 5), 3),
            ("stone", (6, 0, 6), 1),
            ("stone", (6, 2, 5), 7),
            ("stone", (6, 2, 6), 5),
        ] {
            view.place(
                Point::from_absolute(x, y, z).unwrap(),
                tiles.get_by_name(name).unwrap(),
                VariantIndex(variant),
            );
        }

        Self {
            data: Data {
                mesh: view.mesh(ren, Vec3::zero(), ClusterPoint::new(0, 0, 0).unwrap()),
                map,
            },
            cam: TpCamera::new(1., Pnt3::new(3., 0., 3.)),
            aspect: 1.,
        }
    }

    pub fn draw(&mut self, ren: &mut Render, _: f32) {
        ren.set_proj(self.cam.proj(self.aspect));
        ren.set_view(self.cam.view());
        ren.draw(Vec3::new(0.3, 0.6, 0.8), [&self.data as &dyn Pipe])
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn input(&mut self, control: Control) {
        const SENSITIVITY: f32 = 0.01;

        match control {
            Control::Look(x, y) => self.cam.rotate(Vec2::new(x, y) * SENSITIVITY),
            Control::Scroll(_, y) => self.cam.move_to(y),
            Control::Forward => self.cam.move_look(Vec3::new(0., 0., -1.)),
            Control::Back => self.cam.move_look(Vec3::new(0., 0., 1.)),
            Control::Left => self.cam.move_look(Vec3::new(-1., 0., 0.)),
            Control::Right => self.cam.move_look(Vec3::new(1., 0., 0.)),
        }
    }
}
