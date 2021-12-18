use crate::{
    atlas::Atlas,
    camera::TpCamera,
    draw::{cell::Cell, path::Path},
    land::{variant::VariantSet, ClusterView, Factory},
    loader::Loader,
    Render, Texture, Vert,
};
use core::{
    path::{Flyer, PathFinder, Pedestrian, Position},
    prelude::*,
};
use image::DynamicImage;
use ngl::{
    mesh::Indexed,
    pass::{Pass, Solid, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;
use std::{collections::HashMap, rc::Rc, time::Instant};

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
    cells: Vec<Cell>,
    pathes: Vec<Path>,
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
                for variant in &info.variants {
                    let to_variant = loader.load_variant(&variant.name).unwrap();
                    to_variants.push(((info.idx, variant.idx), (to_variant, variant.rotation)));
                }
            }

            loader.take_polygons()
        };

        let atlas = Atlas::new(sprites.iter().map(Rc::as_ref)).unwrap();
        let (map, mapper) = atlas.map();
        let map = ren.make_texture(&map);
        let mut factory = Factory::new(mapper);

        let mut variant_set = VariantSet::new();
        for (key, (to_variant, rotation)) in to_variants {
            let variant = to_variant
                .to_variant(&mut factory, rotation, &mut polygons, |sprite| {
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
            ("dirt", (0, -2, 0), 0),
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
            ("stone", (0, 0, 2), 0),
            ("stone", (1, 0, 2), 0),
            ("stone", (2, 0, 2), 0),
            ("stone", (3, 0, 2), 0),
            ("stone", (3, 1, 1), 0),
            ("stone", (3, 2, 0), 0),
            ("stone", (4, 3, 0), 0),
            ("dirt", (5, 4, 0), 0),
            ("dirt", (6, 4, 0), 2),
            ("ladder", (6, 1, 3), 0),
            ("dirt", (5, 4, 2), 0),
            ("dirt", (6, 4, 2), 0),
            ("dirt", (5, 5, 2), 0),
            ("dirt", (5, 6, 2), 0),
            ("dirt", (5, 7, 2), 0),
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
            ("dirt", (1, 3, 5), 0),
            ("dirt", (2, 1, 5), 0),
            ("dirt", (2, 2, 5), 0),
            ("bricks", (1, 2, 6), 0),
            ("bricks", (1, 4, 6), 0),
            ("grass", (2, 3, 5), 0),
            ("dirt", (0, 0, 6), 1),
            ("dirt", (2, 0, 6), 1),
            ("bricks", (2, 0, 3), 0),
            ("bricks", (3, 0, 3), 0),
            ("bricks", (4, 0, 3), 0),
            ("bricks", (4, 1, 4), 0),
            ("bricks", (5, 2, 4), 0),
            ("bricks", (5, 0, 4), 0),
            ("bricks", (6, 0, 4), 0),
            ("box", (3, 0, 4), 0),
            ("box", (3, 2, 4), 0),
            ("box", (3, 1, 5), 0),
            ("box", (3, 3, 5), 0),
            ("box", (3, 0, 6), 0),
            ("box", (3, 4, 6), 0),
            ("grass", (5, 1, 0), 0),
            ("grass", (5, 1, 2), 1),
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
            ("bricks", (7, 0, 7), 0),
            ("bricks", (7, 0, 6), 0),
            ("bricks", (5, 1, 7), 0),
            ("bricks", (5, 3, 7), 0),
            ("bricks", (7, 0, 7), 0),
            ("bricks", (9, 0, 5), 0),
            ("bricks", (9, 2, 5), 0),
            ("bricks", (9, 4, 5), 0),
            ("bricks", (10, 0, 4), 0),
            ("bricks", (10, 2, 4), 0),
            ("bricks", (10, 4, 4), 0),
            ("bricks", (10, 0, 5), 0),
            ("bricks", (9, 0, 4), 0),
            ("bricks", (9, 2, 4), 0),
            ("bricks", (9, 0, 2), 0),
            ("bricks", (9, 2, 2), 0),
            ("bricks", (9, 0, 0), 0),
            ("bricks", (9, 2, 0), 0),
            ("bricks", (11, 0, 2), 0),
            ("bricks", (11, 2, 2), 0),
            ("bricks", (11, 4, 2), 0),
        ] {
            view.place(
                Point::from_absolute(x, y, z).unwrap(),
                tiles.get_by_name(name).unwrap(),
                VariantIndex(variant),
            );
        }

        let mut pf = PathFinder::new();
        let walk = Flyer {
            walk: Pedestrian {
                height: Height::new(2).unwrap(),
                jump_down: Height::new(4).unwrap(),
            },
        };

        let start = Instant::now();
        pf.find(
            Position {
                pn: Point::from_absolute(3, 2, 3).unwrap(),
                value: 16,
            },
            &walk,
            &view,
        );
        let end = Instant::now();
        println!("elapsed: {} ms", end.duration_since(start).as_millis());

        let path = pf.path();
        let cells = path.points().map(|pn| Cell(pn.into())).collect();

        let mut pathes: Vec<_> = path
            .points()
            .map(|pn| Path(path.to(pn).map(|(_, pn)| pn.into()).collect()))
            .collect();
        pathes.sort_by_key(|path| path.0.len());

        Self {
            data: Data {
                mesh: view.mesh(ren, Vec3::zero(), ClusterPoint::new(0, 0, 0).unwrap()),
                map,
            },
            cells,
            pathes,
            cam: TpCamera::new(1., Pnt3::new(3., 0., 3.)),
            aspect: 1.,
        }
    }

    pub fn draw(&mut self, ren: &mut Render, _: f32) {
        const DRAW_CELLS: bool = false;

        ren.set_proj(self.cam.proj(self.aspect));
        ren.set_view(self.cam.view());

        let cells = DRAW_CELLS.then(|| &self.cells[..]);
        let pathes = &self.pathes[..];
        ren.draw(
            Vec3::new(0.3, 0.6, 0.8),
            [&self.data as &dyn Pipe, &cells, &pathes],
        )
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
