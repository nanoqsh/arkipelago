use crate::{
    atlas::Atlas,
    camera::TpCamera,
    land::{Builder, Factory},
    loader::Loader,
    Render, Texture, Vert,
};
use core::prelude::{ChunkPoint, Side, Sides};
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
        let mut sprite_names = HashMap::new();
        let mut sprites = Vec::new();
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
        loader.on_load_texture(|_, _| ());
        loader.on_load_mesh(|_, _| ());

        loader.load_sprite("tiles/default").unwrap();

        let grass = loader.load_variant("grass").unwrap();
        let polygons = loader.take_polygons();
        drop(loader);

        let atlas = Atlas::new(sprites.iter().map(Rc::as_ref)).unwrap();
        let (map, mapper) = atlas.map();
        let map = ren.make_texture(&map);

        let mut factory = Factory::new(mapper);
        let grass = grass
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

        let mut builder = Builder::with_capacity(64);
        grass.build(
            Vec3::zero(),
            |level, height| {
                let pos = ChunkPoint::new(0, 0, 0).unwrap();
                let variant_height = grass.connections().len() as u8;
                let mut sides = Sides::empty();

                for conn in grass.connections() {
                    let _ = conn.overlaps(conn, Side::Left, &polygons);
                    todo!()
                }

                if level + height == variant_height {
                    sides |= Side::Up;
                } else {
                    match pos.to(Side::Up, level + height) {
                        Ok(hi) => todo!(),
                        Err(hi) => todo!(),
                    }
                }

                if level == 0 {
                    sides |= Side::Down;
                } else {
                    match pos.to(Side::Down, 1) {
                        Ok(lo) => todo!(),
                        Err(lo) => todo!(),
                    }
                }

                for side in [Side::Left, Side::Right, Side::Forth, Side::Back] {
                    let (mut curr, _) = match pos.to(side, 1) {
                        Ok(curr) => (curr, ()),
                        Err(curr) => (curr, ()),
                    };

                    for _ in 0..height {
                        let (next, _) = match curr.to(Side::Up, 1) {
                            Ok(next) => (next, ()),
                            Err(next) => (next, ()),
                        };
                        curr = next;
                    }
                }

                sides
            },
            &mut builder,
        );
        let mesh = builder.mesh(ren);
        builder.clear();

        Self {
            data: Data { mesh, map },
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
