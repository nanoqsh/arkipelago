use crate::{Mesh, Vert};
use core::prelude::*;
use shr::cgm::Vec2;

type Face = [u32; 3];

#[derive(Copy, Clone)]
pub(crate) struct Slotted {
    pub face: Face,
    pub slot: u32,
    pub contact: Sides,
}

pub(crate) struct Shape {
    verts: Box<[Vert]>,
    slotted: Box<[Slotted]>,
    free: Box<[Face]>,
}

impl Shape {
    pub fn new<C, S>(mesh: &Mesh, rotation: Rotation, contact: C, transform_st: S) -> Self
    where
        C: Fn(&str) -> Option<Sides>,
        S: Fn(Vec2) -> Vec2,
    {
        let slots = mesh.slots();
        let mut slotted = Vec::with_capacity(slots.faces_max_len());
        let mut free = Vec::new();
        for (idx, triple) in mesh.indxs().chunks(3).enumerate() {
            let face = match *triple {
                [a, b, c] => [a, b, c],
                _ => unreachable!(),
            };

            match slots.for_face(idx as _) {
                None => free.push(face),
                Some((slot_key, slot)) => {
                    if let Some(contact) = contact(slot_key) {
                        slotted.push(Slotted {
                            face,
                            slot,
                            contact,
                        })
                    }
                }
            }
        }

        Self {
            verts: mesh
                .verts()
                .iter()
                .map(|vert| Vert {
                    co: rotation.transform_vec(vert.co),
                    nm: rotation.transform_vec(vert.nm),
                    st: transform_st(vert.st),
                })
                .collect(),
            slotted: slotted.into_boxed_slice(),
            free: free.into_boxed_slice(),
        }
    }

    pub fn slotted(&self) -> impl Iterator<Item = Slotted> + '_ {
        self.slotted.iter().copied()
    }
}
