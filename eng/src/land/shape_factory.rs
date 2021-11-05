use crate::{atlas::Mapper, land::shape::Shape, Mesh};
use core::prelude::*;
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
        let Parameters {
            mesh,
            rotation,
            discard,
            contact,
        } = params;

        let key = Key {
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
                            contact.get(slot).copied()
                        }
                    },
                    |st| st * self.mapper.multiplier(),
                );
                Rc::clone(en.insert(Rc::new(shape)))
            }
        }
    }
}
