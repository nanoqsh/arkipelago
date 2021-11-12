use crate::{
    land::{builder::Builder, overlay::Overlay, shape::Shape},
    Vert,
};
use core::side::*;
use shr::cgm::*;
use std::{error, fmt, rc::Rc};

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
    overlay: Box<[[Overlay; 6]]>,
    sprite_st: Vec2,
}

impl Variant {
    pub fn new<'a, S>(meshes: S, sprite_st: Vec2) -> Result<Self, Error>
    where
        S: IntoIterator<Item = (Mesh, &'a [[Overlay; 6]])>,
    {
        let mut overlay  = Vec::new();
        Ok(Self {
            meshes: meshes
                .into_iter()
                .map(|(mesh, over)| {
                    overlay.extend_from_slice(over);

                    let n_slots = mesh.sprites_st.len() as u32;
                    if let Some(face) = mesh.shape.slotted().find(|face| face.slot >= n_slots) {
                        return Err(Error::MissedSprite(face.slot));
                    }

                    Ok(mesh)
                })
                .collect::<Result<_, _>>()?,
            overlay: overlay.into_boxed_slice(),
            sprite_st,
        })
    }

    pub fn overlay(&self) -> &[[Overlay; 6]] {
        &self.overlay
    }

    pub fn build<S>(&self, mut offset: Vec3, sides: S, builder: &mut Builder)
    where
        S: Fn(u8, u8) -> Sides,
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
