use crate::{cluster::Cluster, layout::Data, tiles};
use core::{point::Point, prelude::*};
use std::collections::HashMap;

pub struct Placement<'a> {
    pub variant: VariantIndex,
    pub data: &'a [Data],
}

pub trait Tile {
    fn height(&self) -> u8;

    fn variants(&self) -> &[&'static str];

    fn place(&self, cluster: &mut Cluster, pn: Point) -> Placement;
}

pub struct TileSet {
    map: HashMap<String, TileIndex>,
    vec: Vec<Box<dyn Tile>>,
}

impl TileSet {
    pub fn new<'a, T>(tiles: T) -> Self
    where
        T: IntoIterator<Item = &'a TileInfo>,
    {
        let mut tile_set = Self {
            map: HashMap::default(),
            vec: vec![Box::new(tiles::Empty)],
        };

        for info in tiles {
            let name = info.name();
            let tile = Box::new(tiles::Base::new(2, vec!["cube"]));
            tile_set.add(name, tile);
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
        let tile_idx = TileIndex::new(idx as u16).unwrap();
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
        debug_assert_ne!(idx.get(), 0);
        self.vec.get(idx.get() as usize).map(Box::as_ref)
    }
}

impl GetTile<&str> for TileSet {
    fn get_tile(&self, idx: &str) -> Option<&dyn Tile> {
        let tile_idx = *self.map.get(idx)?;
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
            let tile_idx = TileIndex::new(self.idx).unwrap();
            let tile = self.tile_set.get(tile_idx).unwrap();
            self.idx += 1;
            Some((tile_idx, tile))
        } else {
            None
        }
    }
}
