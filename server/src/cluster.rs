use crate::{layout::*, slab::*, tile::*};
use core::{
    map::{Column, Map},
    point::ChunkPoints,
    prelude::*,
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
    column: Column<'a, Slab>,
    chunks: (&'a SlabChunk, Option<&'a SlabChunk>),
}

impl ClusterSlice<'_> {
    pub fn index(&self) -> (TileIndex, VariantIndex) {
        match self.column.get(0).typed() {
            Typed::Base(base) => (base.tile(), base.variant()),
            _ => unreachable!(),
        }
    }

    pub fn data(&self, level: u8) -> Data {
        let level = level as usize;
        let slab = self.column.get(level);
        match slab.typed() {
            Typed::Empty(_) => unreachable!(),
            Typed::Base(_) => Data::None,
            Typed::Trunk(trunk) => {
                let data = trunk.data();
                if trunk.is_obj() {
                    let chunk = if level < self.column.len() {
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
}

pub struct Placed {
    pub variant: VariantIndex,
    pub height: Height,
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

    pub fn get(&self, mut pn: Point) -> Option<(ClusterSlice, u8)> {
        let ch = pn.chunk_point();
        let cl = pn.cluster_point();
        let chunks;
        let (slab, level) = match self.map.get(pn)?.typed() {
            Typed::Empty(_) => return None,
            Typed::Base(slab) => {
                chunks = (self.map.chunk(cl)?, None);
                (slab, 0)
            }
            Typed::Trunk(slab) => {
                let level = slab.level();
                let dw = cl.to(Side::Down);
                pn = match ch.to(Side::Down, level) {
                    Ok(ch) => Point::new(ch, cl),
                    Err(ch) => Point::new(ch, dw),
                };
                let cl = pn.cluster_point();
                chunks = (self.map.chunk(cl)?, self.map.chunk(dw));

                match self.map.get(pn)?.typed() {
                    Typed::Base(slab) => (slab, level),
                    _ => unreachable!(),
                }
            }
        };

        let column = self.map.column(pn, slab.height())?;
        let slice = ClusterSlice { column, chunks };
        debug_assert!(!slice.column.iter().copied().any(Slab::is_empty));
        Some((slice, level))
    }

    pub fn is_empty(&self, pn: Point, height: Height) -> bool {
        match self.map.column(pn, height) {
            Some(column) => column.iter().copied().all(Slab::is_empty),
            None => true,
        }
    }

    pub fn place(&mut self, pn: Point, tile_idx: TileIndex) -> Option<Placed> {
        let tile_set = Rc::clone(&self.tile_set);
        let tile = tile_set.get(tile_idx).unwrap();
        let height = Height::new(tile.height()).unwrap();
        if !self.is_empty(pn, height) {
            return None;
        }

        let placement = tile.place(self, pn);
        assert_eq!(height.get(), placement.data.len() as u8 + 1);
        let layout = Layout {
            tile: tile_idx,
            variant: placement.variant,
            data: placement.data,
        };

        self.map.column_mut(pn, height);
        let cl = pn.cluster_point();
        let storages = (
            self.map.chunk(cl).unwrap().storage.clone(),
            self.map
                .chunk(cl.to(Side::Up))
                .map(|chunk| chunk.storage.clone()),
        );

        let mut column = self.map.column_mut(pn, height);
        *column.get_mut(0) = layout.base().into();
        for (i, (mut trunk, obj)) in layout.trunks().enumerate() {
            let i = i + 1;
            if let Some(obj) = obj {
                if i < column.0.len() {
                    trunk.set_data(storages.0.add(obj))
                } else {
                    trunk.set_data(storages.1.as_ref().unwrap().add(obj))
                }
            }

            *column.get_mut(i) = trunk.into();
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
    type Item = (ClusterSlice<'a>, Point);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.points.next()?;
            let gl = Point::new(ch, self.cl);
            match self.cluster.get(gl) {
                Some((slice, level)) => {
                    debug_assert_eq!(level, 0);
                    for _ in 0..slice.column.0.len() - 1 {
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

        fn place(&self, _: &mut Cluster, _: Point) -> Placement {
            Placement {
                variant: VariantIndex(0),
                data: &self.data,
            }
        }
    }

    fn cluster() -> (Cluster, TileIndex) {
        let mut tile_set = TileSet::new([]);
        let index = tile_set.add(
            "test",
            Box::new(TestTile {
                data: Box::leak(Box::new([
                    Data::None,
                    Data::Num(Num::new(2).unwrap()),
                    Data::Obj(Rc::new(2)),
                ])),
            }),
        );
        (Cluster::new(Rc::new(tile_set)), index)
    }

    #[test]
    fn place() {
        let (mut cluster, index) = cluster();
        let point = Point::from_absolute(0, 0, 0).unwrap();
        cluster.place(point, index);

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.column.len(), 4);
        assert_eq!(slice.index(), (index, VariantIndex(0)));
        assert_eq!(level, 0);

        let point = Point::from_absolute(0, 31, 0).unwrap();
        cluster.place(point, index);

        let (slice, level) = cluster.get(point).unwrap();
        assert_eq!(slice.column.len(), 4);
        assert_eq!(slice.index(), (index, VariantIndex(0)));
        assert_eq!(level, 0);
    }

    #[test]
    fn get() {
        let (mut cluster, index) = cluster();
        cluster.place(Point::from_absolute(0, 0, 0).unwrap(), index);

        for i in 0..4 {
            let (slice, level) = cluster.get(Point::from_absolute(0, i, 0).unwrap()).unwrap();
            assert_eq!(slice.column.len(), 4);
            assert_eq!(slice.index(), (index, VariantIndex(0)));
            assert_eq!(level, i as u8);
        }

        assert!(cluster
            .get(Point::from_absolute(1, 0, 0).unwrap())
            .is_none());
    }

    #[test]
    fn data() {
        let (mut cluster, index) = cluster();
        let point = Point::from_absolute(0, 0, 0).unwrap();
        cluster.place(point, index);

        let (slice, _) = cluster.get(point).unwrap();
        assert_eq!(slice.column.len(), 4);
        assert!(matches!(slice.data(0), Data::None));
        assert_eq!(slice.data(1).as_num().get(), 0);
        assert_eq!(slice.data(2).as_num().get(), 2);
        assert_eq!(slice.data(3).as_obj().downcast_ref::<i32>(), Some(&2));
    }
}
