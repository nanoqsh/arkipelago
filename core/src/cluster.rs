use crate::{layout::Layout, prelude::*, slab::*, tile::Tiles};
use std::{any::Any, collections::HashMap, rc::Rc};

struct SlabChunk {
    slabs: Chunk<Slab>,
    data: Vec<Rc<dyn Any>>,
}

impl SlabChunk {
    fn new() -> Self {
        Self {
            slabs: Chunk::filled(Empty.into()),
            data: Vec::default(),
        }
    }

    fn add_obj(&mut self, obj: Rc<dyn Any>) -> u16 {
        let idx = self.data.len();
        self.data.push(obj);
        idx as u16
    }
}

impl std::ops::Deref for SlabChunk {
    type Target = Chunk<Slab>;

    fn deref(&self) -> &Self::Target {
        &self.slabs
    }
}

impl std::ops::DerefMut for SlabChunk {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.slabs
    }
}

pub struct Cluster<'a> {
    chunks: HashMap<ClusterPoint, SlabChunk>,
    tiles: &'a Tiles,
}

impl<'a> Cluster<'a> {
    pub fn new(tiles: &'a Tiles) -> Self {
        Self {
            chunks: HashMap::default(),
            tiles,
        }
    }

    fn chunk(&mut self, point: ClusterPoint) -> &mut SlabChunk {
        self.chunks.entry(point).or_insert_with(SlabChunk::new)
    }

    pub fn place(&mut self, point: GlobalPoint, tile: u16, variant: u8) {
        let layout = Layout::new(self.tiles, tile, variant);

        let chp = point.chunk_point();
        let clp = point.cluster_point();
        let mut chunk = self.chunk(clp);

        let mut curr = chp;
        *chunk.get_mut(curr) = layout.base().into();
        for (mut trunk, obj) in layout.trunks() {
            curr = match curr.to(Side::Up, 1) {
                Ok(chp) => chp,
                Err(chp) => {
                    chunk = self.chunk(clp.to(Side::Up));
                    chp
                }
            };

            if let Some(obj) = obj {
                trunk.set_data(chunk.add_obj(obj))
            }

            *chunk.get_mut(curr) = trunk.into();
        }
    }
}
