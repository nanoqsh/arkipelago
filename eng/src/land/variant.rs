use crate::land::shape::Shape;
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

pub(crate) enum Slab {
    Empty,
    Mesh {
        shape: Rc<Shape>,
        sprites_st: Box<[Vec2]>,
    },
}

pub(crate) struct Variant {
    slabs: Box<[Slab]>,
    sprite_st: Vec2,
}

impl Variant {
    pub fn new<S>(slabs: S, sprite_st: Vec2) -> Result<Self, Error>
    where
        S: IntoIterator<Item = Slab>,
    {
        let slabs: Result<_, _> = slabs
            .into_iter()
            .map(|slab| {
                match &slab {
                    Slab::Empty => (),
                    Slab::Mesh { shape, sprites_st } => {
                        let n_slots = sprites_st.len() as u32;
                        if let Some(face) = shape.slotted().find(|face| face.slot >= n_slots) {
                            return Err(Error::MissedSprite(face.slot));
                        }
                    }
                }
                Ok(slab)
            })
            .collect();

        Ok(Self {
            slabs: slabs?,
            sprite_st,
        })
    }
}
