use crate::{layout::Data, prelude::*, tiles};
use std::{collections::HashMap, fmt};

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

pub struct TileSet {
    map: HashMap<String, TileIndex>,
    vec: Vec<Box<dyn Tile>>,
}

impl TileSet {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut tile_set = Self {
            map: HashMap::default(),
            vec: vec![Box::new(tiles::Empty)],
        };

        let tiles = [
            ("cube", Box::new(tiles::Base::new(2, vec!["cube"]))),
            ("slab", Box::new(tiles::Base::new(1, vec!["slab"]))),
            ("half", Box::new(tiles::Base::new(1, vec!["half"]))),
            ("bevel_0", Box::new(tiles::Base::new(1, vec!["bevel"]))),
            ("bevel_1", Box::new(tiles::Base::new(1, vec!["bevel_q1"]))),
            ("bevel_2", Box::new(tiles::Base::new(1, vec!["bevel_q2"]))),
            ("bevel_3", Box::new(tiles::Base::new(1, vec!["bevel_q3"]))),
            ("steps_0", Box::new(tiles::Base::new(2, vec!["steps"]))),
            ("steps_1", Box::new(tiles::Base::new(2, vec!["steps_q1"]))),
            ("steps_2", Box::new(tiles::Base::new(2, vec!["steps_q2"]))),
            ("steps_3", Box::new(tiles::Base::new(2, vec!["steps_q3"]))),
        ];

        for (key, tile) in tiles {
            tile_set.add(key, tile);
        }

        tile_set
    }

    pub fn get<I>(&self, index: I) -> Option<&dyn Tile>
    where
        Self: GetTile<I>,
    {
        self.get_tile(index)
    }

    pub fn get_index(&self, key: &str) -> Option<TileIndex> {
        self.map.get(key).copied()
    }

    pub fn tiles(&self) -> Tiles {
        Tiles {
            tile_set: self,
            idx: 1,
        }
    }

    pub fn add<K>(&mut self, key: K, tile: Box<dyn Tile>) -> TileIndex
    where
        K: Into<String>,
    {
        let idx = self.vec.len();
        assert!(idx <= u16::MAX as usize);
        let tile_idx = TileIndex(idx as u16);
        let old = self.map.insert(key.into(), tile_idx);
        assert!(old.is_none());
        self.vec.push(tile);
        tile_idx
    }
}

pub trait GetTile<I> {
    fn get_tile(&self, idx: I) -> Option<&dyn Tile>;
}

impl GetTile<TileIndex> for TileSet {
    fn get_tile(&self, idx: TileIndex) -> Option<&dyn Tile> {
        debug_assert_ne!(idx.0, 0);
        self.vec.get(idx.0 as usize).map(Box::as_ref)
    }
}

impl<T: AsRef<str>> GetTile<T> for TileSet {
    fn get_tile(&self, idx: T) -> Option<&dyn Tile> {
        let tile_idx = *self.map.get(idx.as_ref())?;
        self.get_tile(tile_idx)
    }
}

pub struct Tiles<'a> {
    tile_set: &'a TileSet,
    idx: u16,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = (TileIndex, &'a dyn Tile);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.tile_set.vec.len() as u16 {
            let tile_idx = TileIndex(self.idx);
            let tile = self.tile_set.get(tile_idx).unwrap();
            self.idx += 1;
            Some((tile_idx, tile))
        } else {
            None
        }
    }
}
