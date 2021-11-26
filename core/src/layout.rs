use crate::{tile::TileIndex, slab::*};
use std::{any::Any, rc::Rc};

#[derive(Copy, Clone)]
pub struct Num(u16);

impl Num {
    pub const fn new(num: u16) -> Option<Self> {
        if num >= 1 << 12 {
            None
        } else {
            Some(Self(num))
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

pub enum Data {
    None,
    Num(Num),
    Obj(Rc<dyn Any>),
}

#[derive(Copy, Clone)]
pub(crate) struct Layout<'a> {
    pub tile: TileIndex,
    pub variant: u8,
    pub data: &'a [Data],
}

impl<'a> Layout<'a> {
    pub fn height(&self) -> u8 {
        self.data.len() as u8 + 1
    }

    pub fn base(self) -> Base {
        Base::new(self.tile, self.variant, self.height())
    }

    pub fn trunks(self) -> impl Iterator<Item = (Trunk, Option<Rc<dyn Any>>)> + 'a {
        self.data.iter().enumerate().map(move |(i, data)| {
            let level = i as u8 + 1;
            match data {
                Data::None => (Trunk::new(self.tile, 0, false, level), None),
                Data::Num(num) => (Trunk::new(self.tile, num.get(), false, level), None),
                Data::Obj(obj) => (Trunk::new(self.tile, 0, true, level), Some(Rc::clone(obj))),
            }
        })
    }
}
