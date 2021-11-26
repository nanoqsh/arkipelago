use crate::{layout::Data, prelude::*};
use std::fmt;

pub struct Placement<'a> {
    pub variant: VariantIndex,
    pub data: &'a [Data],
}

pub trait Tile {
    fn height(&self) -> u8;

    fn variants(&self) -> &[&'static str];

    fn place(&self, cluster: &mut Cluster, point: GlobalPoint) -> Placement;
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct TileIndex(pub(crate) u16);

impl TileIndex {
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for TileIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tile: {}", self.0)
    }
}

impl fmt::Debug for TileIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct VariantIndex(pub u8);

impl VariantIndex {
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for VariantIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "variant: {}", self.0)
    }
}

impl fmt::Debug for VariantIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct TileSet(Vec<Box<dyn Tile>>);

impl TileSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![Box::new(crate::tiles::Empty)])
    }

    pub fn get(&self, tile: TileIndex) -> &dyn Tile {
        debug_assert_ne!(tile.0, 0);
        self.0[tile.0 as usize].as_ref()
    }

    pub fn add(&mut self, tile: Box<dyn Tile>) -> TileIndex {
        let idx = self.0.len();
        assert!(idx <= u16::MAX as usize);
        self.0.push(tile);
        TileIndex(idx as u16)
    }
}
