use crate::{
    atlas::Mapper,
    land::builder::{Builder, VertexData},
    Mesh, Vert,
};
use core::prelude::*;
use shr::cgm::Vec2;
use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap, HashSet},
    rc::Rc,
};

pub(crate) struct Parameters<'a> {
    pub mesh: &'a Mesh,
    pub rotation: Rotation,
    pub discard: &'a HashSet<String>,
    pub contact: &'a HashMap<String, Sides>,
}

#[derive(Eq, Hash, PartialEq)]
struct Key {
    mesh: *const Mesh,
    rotation: Rotation,
    discard: BTreeSet<u32>,
}

pub(crate) struct Factory {
    shapes: HashMap<Key, Rc<Shape>>,
    mapper: Mapper,
}

impl Factory {
    pub fn new(mapper: Mapper) -> Self {
        Self {
            shapes: HashMap::with_capacity(16),
            mapper,
        }
    }

    pub fn make(&mut self, params: Parameters) -> Rc<Shape> {
        const SHIFT: f32 = 0.00001;

        let Parameters {
            mesh,
            rotation,
            discard,
            contact,
        } = params;

        let key = Key {
            mesh,
            rotation,
            discard: mesh
                .slots()
                .values()
                .filter_map(|(slot, index)| discard.contains(slot).then(|| index))
                .collect(),
        };

        match self.shapes.entry(key) {
            Entry::Occupied(en) => Rc::clone(en.get()),
            Entry::Vacant(en) => {
                let shape = Shape::new(
                    mesh,
                    rotation,
                    |slot| {
                        if discard.contains(slot) {
                            None
                        } else {
                            Some(contact.get(slot).copied().unwrap_or_default())
                        }
                    },
                    |st| {
                        (st * (1. - 2. * SHIFT) + Vec2::new(SHIFT, SHIFT))
                            * self.mapper.multiplier()
                    },
                );
                Rc::clone(en.insert(Rc::new(shape)))
            }
        }
    }
}

type Face = [u32; 3];

#[derive(Copy, Clone)]
pub(crate) struct Slotted {
    pub face: Face,
    pub slot: u32,
    pub contact: Sides,
}

impl From<Slotted> for VertexData {
    fn from(Slotted { face, slot, .. }: Slotted) -> Self {
        Self { face, slot }
    }
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
                            contact: contact
                                .into_iter()
                                .map(|side| rotation.rotate(side))
                                .collect(),
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

    pub fn build<V>(&self, sides: Sides, vertex: V, builder: &mut Builder)
    where
        V: Fn(Vert, u32) -> Vert,
    {
        let free = self.free.iter().copied().map(|face| VertexData {
            face,
            slot: u32::MAX,
        });

        let slotted = self
            .slotted
            .iter()
            .filter(|face| {
                let contact = face.contact;
                sides & contact == contact
            })
            .copied()
            .map(Into::into);

        builder.extend(free.chain(slotted), |vert_idx, slot| {
            let vert = unsafe {
                let vert_idx = vert_idx as usize;
                debug_assert!(vert_idx < self.verts.len());
                *self.verts.get_unchecked(vert_idx)
            };

            vertex(vert, slot)
        })
    }
}
