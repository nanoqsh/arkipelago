use crate::{
    land::{builder::Builder, shape::Shape, Connections},
    Vert,
};
use core::prelude::*;
use shr::cgm::*;
use std::{collections::HashMap, error, fmt, rc::Rc};

#[derive(Debug)]
pub(crate) enum Error {
    MissedSprite(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissedSprite(sprite) => write!(f, "missed sprite {}", sprite),
        }
    }
}

impl error::Error for Error {}

pub(crate) struct Mesh {
    pub shape: Rc<Shape>,
    pub sprites_st: Box<[Vec2]>,
    pub height: u8,
}

pub(crate) struct Variant {
    meshes: Box<[Mesh]>,
    conn: Box<[Connections]>,
    sprite_st: Vec2,
}

impl Variant {
    pub fn new<S, C>(meshes: S, sprite_st: Vec2) -> Result<Self, Error>
    where
        S: IntoIterator<Item = (Mesh, C)>,
        C: IntoIterator<Item = Connections>,
    {
        let mut conn = Vec::new();
        Ok(Self {
            meshes: meshes
                .into_iter()
                .map(|(mesh, cs)| {
                    conn.extend(cs);

                    let n_slots = mesh.sprites_st.len() as u32;
                    if let Some(face) = mesh.shape.slotted().find(|face| face.slot >= n_slots) {
                        return Err(Error::MissedSprite(face.slot));
                    }

                    Ok(mesh)
                })
                .collect::<Result<_, _>>()?,
            conn: conn.into_boxed_slice(),
            sprite_st,
        })
    }

    pub fn height(&self) -> u8 {
        self.conn.len() as u8
    }

    pub fn connections(&self) -> &[Connections] {
        &self.conn
    }

    pub fn build<S>(&self, mut offset: Vec3, mut sides: S, builder: &mut Builder)
    where
        S: FnMut(u8, u8) -> Sides,
    {
        let mut level = 0;
        for mesh in self.meshes.iter() {
            mesh.shape.build(
                sides(level, mesh.height),
                |vert, slot| Vert {
                    co: vert.co + offset,
                    nm: vert.nm,
                    st: vert.st
                        + match slot {
                            u32::MAX => self.sprite_st,
                            _ => mesh.sprites_st[slot as usize],
                        },
                },
                builder,
            );

            level += mesh.height;
            offset.y += 0.5 * mesh.height as f32;
        }
    }
}

pub(crate) struct VariantSet {
    variants: HashMap<(TileIndex, VariantIndex), Variant>,
}

impl VariantSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            variants: HashMap::default(),
        }
    }

    pub fn get(&self, key: (TileIndex, VariantIndex)) -> &Variant {
        self.variants.get(&key).unwrap()
    }

    pub fn add(&mut self, key: (TileIndex, VariantIndex), variant: Variant) {
        self.variants.entry(key).or_insert(variant);
    }
}
