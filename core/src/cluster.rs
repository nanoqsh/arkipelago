use crate::{
    layout::{Data, Layout, Num},
    map::Map,
    point::ChunkPoints,
    prelude::*,
    slab::*,
    tile::TileSet,
};
use std::{any::Any, cell::RefCell, rc::Rc};

#[derive(Clone)]
struct Storage {
    data: Rc<RefCell<Vec<Rc<dyn Any>>>>,
}

impl Storage {
    fn new() -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::default())),
        }
    }

    fn get(&self, idx: u16) -> Rc<dyn Any> {
        let data = self.data.borrow();
        Rc::clone(&data[idx as usize])
    }

    fn add(&self, obj: Rc<dyn Any>) -> u16 {
        let mut data = self.data.borrow_mut();
        let idx = data.len();
        data.push(obj);
        idx as u16
    }
}

struct SlabChunk {
    slabs: Chunk<Slab>,
    storage: Storage,
}

impl SlabChunk {
    fn new() -> Self {
        Self {
            slabs: Chunk::filled(Empty.into()),
            storage: Storage::new(),
        }
    }
}

impl Default for SlabChunk {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<Chunk<Slab>> for SlabChunk {
    fn as_ref(&self) -> &Chunk<Slab> {
        &self.slabs
    }
}

impl AsMut<Chunk<Slab>> for SlabChunk {
    fn as_mut(&mut self) -> &mut Chunk<Slab> {
        &mut self.slabs
    }
}

pub struct ClusterSlice<'a> {
    lo: &'a [Slab],
    hi: &'a [Slab],
    chunks: (&'a SlabChunk, Option<&'a SlabChunk>),
}

impl<'a> ClusterSlice<'a> {
    pub const fn len(&self) -> usize {
        self.lo.len() + self.hi.len()
    }

    pub const fn is_empty(&self) -> bool {
        false
    }

    pub fn index(&self) -> (TileIndex, VariantIndex) {
        match self.lo[0].typed() {
            Typed::Base(base) => (base.tile(), base.variant()),
            _ => unreachable!(),
        }
    }

    pub fn data(&self, level: u8) -> Data {
        let level = level as usize;
        let slab = self.get(level);
        match slab.typed() {
            Typed::Empty(_) => unreachable!(),
            Typed::Base(_) => Data::None,
            Typed::Trunk(trunk) => {
                let data = trunk.data();
                if trunk.is_obj() {
                    let chunk = if level < self.lo.len() {
                        self.chunks.0
                    } else {
                        self.chunks.1.unwrap()
                    };

                    Data::Obj(chunk.storage.get(data))
                } else {
                    Data::Num(Num::new(data).unwrap())
                }
            }
        }
    }

    fn get(&self, level: usize) -> Slab {
        *self
            .lo
            .get(level)
            .or_else(|| self.hi.get(level - self.lo.len()))
            .unwrap()
    }
}

pub struct Placed {
    pub variant: VariantIndex,
    pub height: u8,
}

pub struct Cluster {
    map: Map<SlabChunk>,
    tile_set: Rc<TileSet>,
}

impl Cluster {
    pub fn new(tile_set: Rc<TileSet>) -> Self {
        Self {
            map: Map::default(),
            tile_set,
        }
    }

    pub fn tiles(&self, cl: ClusterPoint) -> Option<Tiles> {
        Some(Tiles {
            cluster: self,
            points: ChunkPoints::new(),
            cl,
        })
    }

    pub fn get(&self, mut gl: GlobalPoint) -> Option<(ClusterSlice, u8)> {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let chunks;
        let (slab, level) = match self.map.get(gl)?.typed() {
            Typed::Empty(_) => return None,
            Typed::Base(slab) => {
                chunks = (self.map.chunk(cl)?, None);
                (slab, 0)
            }
            Typed::Trunk(slab) => {
                let level = slab.level();
                let dw = cl.to(Side::Down);
                gl = match ch.to(Side::Down, level) {
                    Ok(ch) => GlobalPoint::new(ch, cl),
                    Err(ch) => GlobalPoint::new(ch, dw),
                };
                let cl = gl.cluster_point();
                chunks = (self.map.chunk(cl)?, self.map.chunk(dw));

                match self.map.get(gl)?.typed() {
                    Typed::Base(slab) => (slab, level),
                    _ => unreachable!(),
                }
            }
        };

        let (lo, hi) = self.map.slice(gl, slab.height())?;
        let slice = ClusterSlice { lo, hi, chunks };
        debug_assert!((0..slice.len()).all(|l| !slice.get(l).is_empty()));
        Some((slice, level))
    }

    pub fn place(&mut self, gl: GlobalPoint, tile: TileIndex) -> Option<Placed> {
        let tiles = Rc::clone(&self.tile_set);
        let tile_obj = tiles.get(tile);
        let height = tile_obj.height();
        let (lo, hi) = self.map.slice_mut(gl, height);
        if !lo.iter().chain(hi.iter()).copied().all(Slab::is_empty) {
            return None;
        }

        let placement = tile_obj.place(self, gl);
        assert_eq!(height, placement.data.len() as u8 + 1);
        let layout = Layout {
            tile,
            variant: placement.variant,
            data: placement.data,
        };

        let cl = gl.cluster_point();
        let storages = (
            self.map.chunk(cl).unwrap().storage.clone(),
            self.map
                .chunk(cl.to(Side::Up))
                .map(|chunk| chunk.storage.clone()),
        );

        let (lo, hi) = self.map.slice_mut(gl, height);
        lo[0] = layout.base().into();
        for (i, (mut trunk, obj)) in layout.trunks().enumerate() {
            let i = i + 1;
            if i < lo.len() {
                lo[i] = trunk.into();

                if let Some(obj) = obj {
                    trunk.set_data(storages.0.add(obj))
                }
            } else {
                let i = i - lo.len();
                hi[i] = trunk.into();

                if let Some(obj) = obj {
                    trunk.set_data(storages.1.as_ref().unwrap().add(obj))
                }
            }
        }

        Some(Placed {
            variant: placement.variant,
            height,
        })
    }
}

pub struct Tiles<'a> {
    cluster: &'a Cluster,
    points: ChunkPoints,
    cl: ClusterPoint,
}

impl<'a> Iterator for Tiles<'a> {
    type Item = (ClusterSlice<'a>, GlobalPoint);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.points.next()?;
            let gl = GlobalPoint::new(ch, self.cl);
            match self.cluster.get(gl) {
                Some((slice, level)) => {
                    debug_assert_eq!(level, 0);
                    for _ in 0..slice.lo.len() - 1 {
                        self.points.next();
                    }

                    break Some((slice, gl));
                }
                None => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestTile {
        data: &'static [Data],
    }

    impl Tile for TestTile {
        fn height(&self) -> u8 {
            4
        }

        fn variants(&self) -> &[&'static str] {
            unreachable!()
        }

        fn place(&self, _: &mut Cluster, _: GlobalPoint) -> Placement {
            Placement {
                variant: VariantIndex(0),
                data: &self.data,
            }
        }
    }

    fn cluster() -> (Cluster, TileIndex) {
        let mut tile_set = TileSet::new();
        let index = tile_set.add(Box::new(TestTile {
            data: Box::leak(Box::new([
                Data::None,
                Data::Num(Num::new(2).unwrap()),
                Data::Obj(Rc::new(2)),
            ])),
        }));
        (Cluster::new(Rc::new(tile_set)), index)
    }

    #[test]
    fn place() {
        let (mut cluster, index) = cluster();
        let point = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        cluster.place(point, index);

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.index(), (index, VariantIndex(0)));
        assert_eq!(level, 0);

        let point = GlobalPoint::from_absolute(0, 31, 0).unwrap();
        cluster.place(point, index);

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.len(), 4);
        assert_eq!(slice.index(), (index, VariantIndex(0)));
        assert_eq!(level, 0);
    }

    #[test]
    fn get() {
        let (mut cluster, index) = cluster();
        cluster.place(GlobalPoint::from_absolute(0, 0, 0).unwrap(), index);

        for i in 0..4 {
            let (slice, level) = cluster
                .get(GlobalPoint::from_absolute(0, i, 0).unwrap())
                .unwrap();
            assert_eq!(slice.len(), 4);
            assert_eq!(slice.index(), (index, VariantIndex(0)));
            assert_eq!(level, i as u8);
        }

        assert!(cluster
            .get(GlobalPoint::from_absolute(1, 0, 0).unwrap())
            .is_none());
    }

    #[test]
    fn data() {
        let (mut cluster, index) = cluster();
        let point = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        cluster.place(point, index);

        let (slice, _) = cluster.get(point).unwrap();
        assert_eq!(slice.len(), 4);
        assert!(matches!(slice.data(0), Data::None));
        assert_eq!(slice.data(1).as_num().get(), 0);
        assert_eq!(slice.data(2).as_num().get(), 2);
        assert_eq!(slice.data(3).as_obj().downcast_ref::<i32>(), Some(&2));
    }
}
