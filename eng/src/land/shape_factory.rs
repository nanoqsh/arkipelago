use crate::{atlas::Mapper, land::shape::Shape, Mesh};
use core::prelude::*;
use std::{
    collections::{hash_map::Entry, BTreeSet, HashMap},
    rc::Rc,
};

pub(crate) struct Parameters<'a, C> {
    pub mesh: &'a Mesh,
    pub rotation: Rotation,
    pub contact: C,
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

    pub fn make<C>(&mut self, params: Parameters<C>) -> Rc<Shape>
    where
        C: Fn(&str) -> Option<Sides>,
    {
        let Parameters {
            mesh,
            rotation,
            contact,
        } = params;

        let key = Key {
            rotation,
            discard: mesh
                .slots()
                .values()
                .filter_map(|(slot, index)| contact(slot).is_none().then(|| index))
                .collect(),
        };

        match self.shapes.entry(key) {
            Entry::Occupied(en) => Rc::clone(en.get()),
            Entry::Vacant(en) => {
                let shape = Shape::new(mesh, rotation, contact, |st| st * self.mapper.multiplier());
                Rc::clone(en.insert(Rc::new(shape)))
            }
        }
    }
}
