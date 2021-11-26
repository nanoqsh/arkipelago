use crate::{
    atlas::Atlas,
    camera::TpCamera,
    land::{variant::VariantSet, Factory},
    loader::Loader,
    view::ClusterView,
    Render, Texture, Vert,
};
use core::{prelude::*, tiles};
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
        let tiles = [
            ("tile0", Box::new(tiles::Cube::new(vec!["grass"]))),
            ("tile1", Box::new(tiles::Cube::new(vec!["grass"]))),
            ("tile2", Box::new(tiles::Cube::new(vec!["grass"]))),
        ];

        let mut names_tiles = HashMap::new();
        let mut tile_set = TileSet::new();
        let mut to_variants = Vec::new();

        let mut sprite_names = HashMap::new();
        let mut sprites = Vec::new();

        let polygons = {
            let mut loader = Loader::new(ren);
            loader.on_load_sprite(|name, sprite: Rc<DynamicImage>| {
                let (_, name) = name.split_once('/').unwrap();
                if sprites.is_empty() {
                    debug_assert_eq!(name, "default")
                }

                assert!(sprite_names
                    .insert(name.to_string(), sprites.len() as u32)
                    .is_none());
                sprites.push(sprite);
            });
            loader.load_sprite("tiles/default").unwrap();

            for (tile_name, tile) in tiles {
                let index = tile_set.add(tile);
                names_tiles.insert(tile_name, index);
                for (i, variant_name) in tile_set.get(index).variants().iter().enumerate() {
                    let to_variant = loader.load_variant(variant_name).unwrap();
                    to_variants.push(((index, VariantIndex(i as u8)), to_variant));
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
                .to_variant(&mut factory, |sprite| {
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

        let mut view = ClusterView::new(tile_set, variant_set, polygons);
        for (x, y, z) in [
            (0, 0, 0),
            (1, 0, 0),
            (0, 0, 1),
            (0, 4, 0),
            (0, 4, 4),
            (4, 4, 0),
            (4, 4, 4),
            (4, 16, 4),
        ] {
            view.place(
                GlobalPoint::from_absolute(x, y, z).unwrap(),
                names_tiles["tile0"],
            );
        }

        Self {
            data: Data {
                mesh: view.mesh(ren, Vec3::zero(), ClusterPoint::new(0, 0, 0).unwrap()),
                map,
            },
            cam: TpCamera::new(1., Pnt3::origin()),
            aspect: 1.,
        }
    }

    pub fn draw(&mut self, ren: &mut Render, _: f32) {
        ren.set_proj(self.cam.proj(self.aspect));
        ren.set_view(self.cam.view());
        ren.draw([&self.data as &dyn Pipe])
    }

    pub fn resize(&mut self, (width, height): (u32, u32)) {
        self.aspect = width as f32 / height as f32;
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
