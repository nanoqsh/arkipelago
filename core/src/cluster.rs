use crate::{prelude::*, slab::Slab};
use std::collections::HashMap;

pub struct Cluster {
    chunks: HashMap<ClusterPoint, Chunk<Slab>>,
}

impl Cluster {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }

    pub fn set(&mut self, point: GlobalPoint, tile: &dyn Tile, variant: u8) {
        todo!()
    }
}
