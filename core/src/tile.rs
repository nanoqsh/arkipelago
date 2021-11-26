use crate::{layout::Data, prelude::*};
use std::fmt;

pub struct Placement<'a> {
    pub variant: u8,
    pub data: &'a [Data],
}

pub trait Tile {
    fn height(&self) -> u8;

    fn place(&self, cluster: &mut Cluster, point: GlobalPoint) -> Placement;
}

#[derive(Copy, Clone, Eq, PartialEq)]
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

pub struct Tiles(Vec<Box<dyn Tile>>);

impl Tiles {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(vec![Box::new(crate::tiles::Empty)])
    }

    pub fn get(&self, tile: TileIndex) -> &dyn Tile {
        debug_assert_ne!(tile.0, 0);
        self.0[tile.0 as usize].as_ref()
    }

    pub fn add<T>(&mut self, tile: T) -> TileIndex
    where
        T: Tile + 'static,
    {
        let idx = self.0.len();
        assert!(idx <= u16::MAX as usize);
        self.0.push(Box::new(tile));
        TileIndex(idx as u16)
    }
}
