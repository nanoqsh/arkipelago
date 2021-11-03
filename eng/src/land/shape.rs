use crate::{Mesh, Vert};
use core::prelude::*;

type Face = [u32; 3];

struct Slotted {
    face: Face,
    slot: u32,
    contact: Sides,
}

pub(crate) struct Shape {
    verts: Box<[Vert]>,
    slotted: Box<[Slotted]>,
    free: Box<[Face]>,
}

impl Shape {
    pub fn new<C>(mesh: &Mesh, rotation: Rotation, contact: C) -> Self
    where
        C: Fn(u32) -> Option<Sides>,
    {
        let slots = mesh.slots();
        let mut slotted = Vec::with_capacity(slots.faces_max_len());
        let mut free = Vec::new();
        for (idx, triple) in mesh.indxs().chunks(3).enumerate() {
            let face = match *triple {
                [a, b, c] => [a, b, c],
                _ => unreachable!(),
            };

            match slots.slots_for(idx as _).next() {
                None => free.push(face),
                Some(slot) => {
                    let slot = slot as _;
                    if let Some(contact) = contact(slot) {
                        slotted.push(Slotted {
                            face,
                            slot,
                            contact,
                        });
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
                    st: vert.st,
                })
                .collect(),
            slotted: slotted.into_boxed_slice(),
            free: free.into_boxed_slice(),
        }
    }
}
