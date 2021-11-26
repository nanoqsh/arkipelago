use crate::{chunk::HEIGHT, layout::Layout, prelude::*, slab::*, tile::Tiles};
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

pub struct ClusterSlice<'a> {
    lo: &'a [Slab],
    hi: &'a [Slab],
}

impl<'a> ClusterSlice<'a> {
    pub fn len(&self) -> usize {
        self.lo.len() + self.hi.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn tile(&self) -> TileIndex {
        match self.lo[0].typed() {
            Typed::Base(base) => base.tile(),
            _ => unreachable!(),
        }
    }
}

pub struct Cluster {
    chunks: HashMap<ClusterPoint, SlabChunk>,
    tiles: Rc<Tiles>,
}

impl Cluster {
    pub fn new(tiles: Rc<Tiles>) -> Self {
        Self {
            chunks: HashMap::default(),
            tiles,
        }
    }

    pub fn get(&self, point: GlobalPoint) -> Option<(ClusterSlice, u8)> {
        let chp = point.chunk_point();
        let clp = point.cluster_point();

        let mut chunk = self.chunks.get(&clp)?;
        let mut curr = chp;
        let (slab, level) = match chunk.get(curr).typed() {
            Typed::Empty(_) => return None,
            Typed::Base(slab) => (slab, 0),
            Typed::Trunk(slab) => {
                let level = slab.level();
                curr = match chp.to(Side::Down, level) {
                    Ok(chp) => chp,
                    Err(chp) => {
                        chunk = self.chunks.get(&clp.to(Side::Down))?;
                        chp
                    }
                };

                match chunk.get(chp).typed() {
                    Typed::Base(slab) => (slab, level),
                    _ => unreachable!(),
                }
            }
        };

        let height = slab.height();
        let u = curr.y().saturating_add(height);
        let slice = if u <= HEIGHT as u8 {
            ClusterSlice {
                lo: chunk.slice(curr, height),
                hi: &[],
            }
        } else {
            let hh = u - HEIGHT as u8;
            let lh = height - hh;
            let (x, _, z) = curr.axes();
            ClusterSlice {
                lo: chunk.slice(curr, lh),
                hi: self
                    .chunks
                    .get(&clp.to(Side::Up))?
                    .slice(ChunkPoint::new(x, 0, z).unwrap(), hh),
            }
        };

        Some((slice, level))
    }

    pub fn place(&mut self, point: GlobalPoint, tile: TileIndex) -> bool {
        let chp = point.chunk_point();
        let clp = point.cluster_point();
        let tiles = Rc::clone(&self.tiles);
        let tile_obj = tiles.get(tile);

        let mut chunk = self.chunk(clp);
        let mut curr = chp;
        let height = tile_obj.height();
        for _ in 0..height {
            curr = match curr.to(Side::Up, 1) {
                Ok(chp) => chp,
                Err(chp) => {
                    chunk = self.chunk(clp.to(Side::Up));
                    chp
                }
            };

            if !chunk.get(curr).is_empty() {
                return false;
            }
        }

        let placement = tile_obj.place(self, point);
        assert_eq!(height, placement.data.len() as u8 + 1);
        let layout = Layout {
            tile,
            variant: placement.variant,
            data: placement.data,
        };

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

        true
    }

    fn chunk(&mut self, point: ClusterPoint) -> &mut SlabChunk {
        self.chunks.entry(point).or_insert_with(SlabChunk::new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Data, Num};

    struct TestTile {
        data: &'static [Data],
    }

    impl Tile for TestTile {
        fn height(&self) -> u8 {
            4
        }

        fn place(&self, _: &mut Cluster, _: GlobalPoint) -> Placement {
            Placement {
                variant: 0,
                data: &self.data,
            }
        }
    }

    #[test]
    fn cluster() {
        let mut tiles = Tiles::new();
        let index = tiles.add(TestTile {
            data: Box::leak(Box::new([
                Data::None,
                Data::Num(Num::new(2).unwrap()),
                Data::Obj(Rc::new(2)),
            ])),
        });
        let mut cluster = Cluster::new(Rc::new(tiles));
        let point = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        assert!(cluster.place(point, index));
        assert!(!cluster.place(point, index));

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.tile(), index);
        assert_eq!(level, 0);

        assert!(cluster
            .get(GlobalPoint::from_absolute(1, 0, 0).unwrap())
            .is_none());
        assert!(cluster
            .get(GlobalPoint::from_absolute(0, 0, 1).unwrap())
            .is_none());

        let point = GlobalPoint::from_absolute(0, 31, 0).unwrap();
        assert!(cluster.place(point, index));

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.tile(), index);
        assert_eq!(level, 0);
    }
}
