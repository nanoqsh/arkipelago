use crate::{
    land::{
        polygon::{Axis, Polygons},
        variant::VariantSet,
        Builder, Connections,
    },
    IndexedMesh, Render,
};
use core::{
    map::{Column, Map},
    path::{Pass, Space},
    prelude::*,
};
use shr::cgm::Vec3;

#[derive(Copy, Clone)]
enum Slab {
    Empty,
    Base(TileIndex, VariantIndex),
    Trunk(u8),
}

struct Data {
    keys: Chunk<Slab>,
    connections: Chunk<Connections>,
    passes: Chunk<Pass>,
}

impl Data {
    fn connection(&self, ch: ChunkPoint) -> &Connections {
        self.connections.get(ch)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self {
            keys: Chunk::filled(Slab::Empty),
            connections: Chunk::filled(Connections::new()),
            passes: Chunk::filled(Pass::empty()),
        }
    }
}

impl From<(TileIndex, VariantIndex)> for Slab {
    fn from((tile, variant): (TileIndex, VariantIndex)) -> Self {
        Self::Base(tile, variant)
    }
}

impl AsRef<Chunk<Slab>> for Data {
    fn as_ref(&self) -> &Chunk<Slab> {
        &self.keys
    }
}

impl AsMut<Chunk<Slab>> for Data {
    fn as_mut(&mut self) -> &mut Chunk<Slab> {
        &mut self.keys
    }
}

impl AsRef<Chunk<Connections>> for Data {
    fn as_ref(&self) -> &Chunk<Connections> {
        &self.connections
    }
}

impl AsMut<Chunk<Connections>> for Data {
    fn as_mut(&mut self) -> &mut Chunk<Connections> {
        &mut self.connections
    }
}

impl AsRef<Chunk<Pass>> for Data {
    fn as_ref(&self) -> &Chunk<Pass> {
        &self.passes
    }
}

impl AsMut<Chunk<Pass>> for Data {
    fn as_mut(&mut self) -> &mut Chunk<Pass> {
        &mut self.passes
    }
}

pub(crate) struct ClusterView {
    map: Map<Data>,
    variant_set: VariantSet,
    polygons: Polygons,
    builder: Builder,
}

impl ClusterView {
    pub fn new(variant_set: VariantSet, polygons: Polygons) -> Self {
        Self {
            map: Map::default(),
            variant_set,
            polygons,
            builder: Builder::with_capacity(64),
        }
    }

    pub fn place(&mut self, pn: Point, tile: &TileInfo, variant: VariantIndex) {
        let height = tile.height;
        let mut column = self.map.column_mut(pn, height);
        let key = (tile.idx, variant);
        *column.get_mut(0) = key.into();
        for (i, slab) in column.iter_mut().skip(1).enumerate() {
            *slab = Slab::Trunk(i as u8)
        }

        let variant = tile.variant(variant);
        let mut column = self.map.column_mut(pn, height);
        for (dst, src) in column.iter_mut().zip(&variant.passes) {
            *dst = *src;
        }

        let variant = self.variant_set.get(key);
        let mut column = self.map.column_mut(pn, height);
        for (dst, src) in column.iter_mut().zip(variant.connections()) {
            *dst = *src;
        }
    }

    pub fn mesh(&mut self, ren: &Render, offset: Vec3, cl: ClusterPoint) -> IndexedMesh {
        let builder = &mut self.builder;
        let mut vicinity = self.map.vicinity(cl).unwrap();
        for (slab, ch) in self.map.iter(cl).unwrap() {
            let key = match *slab {
                Slab::Base(tile, variant) => (tile, variant),
                _ => continue,
            };

            let local_offset: Vec3 = ch.into();
            let variant = self.variant_set.get(key);
            let variant_height = variant.height();
            let connections = variant.connections();

            variant.build(
                offset + local_offset,
                |level, shape_height| {
                    let mut sides = Sides::empty();
                    let ch = ch.to(Side::Up, level).unwrap();

                    if level + shape_height == variant_height {
                        match ch.to(Side::Up, shape_height) {
                            Ok(hi) => {
                                let other = vicinity.center().connection(hi);
                                if !other.overlaps(
                                    connections.last().unwrap(),
                                    Side::Down,
                                    &self.polygons,
                                    Axis::Y,
                                ) {
                                    sides |= Side::Up;
                                }
                            }
                            Err(hi) => match vicinity.from(Side::Up) {
                                Some(from) => {
                                    let other = from.connection(hi);
                                    if !other.overlaps(
                                        connections.last().unwrap(),
                                        Side::Down,
                                        &self.polygons,
                                        Axis::Y,
                                    ) {
                                        sides |= Side::Up;
                                    }
                                }
                                None => sides |= Side::Up,
                            },
                        }
                    } else {
                        sides |= Side::Up;
                    }

                    if level == 0 {
                        match ch.to(Side::Down, 1) {
                            Ok(lo) => {
                                let other = vicinity.center().connection(lo);
                                if !other.overlaps(
                                    connections.first().unwrap(),
                                    Side::Up,
                                    &self.polygons,
                                    Axis::Y,
                                ) {
                                    sides |= Side::Down;
                                }
                            }
                            Err(lo) => match vicinity.from(Side::Down) {
                                Some(from) => {
                                    let other = from.connection(lo);
                                    if !other.overlaps(
                                        connections.first().unwrap(),
                                        Side::Up,
                                        &self.polygons,
                                        Axis::Y,
                                    ) {
                                        sides |= Side::Down;
                                    }
                                }
                                None => sides |= Side::Down,
                            },
                        }
                    } else {
                        sides |= Side::Down;
                    }

                    for side in [Side::Left, Side::Right, Side::Forth, Side::Back] {
                        let (mut curr, mut other) = match ch.to(side, 1) {
                            Ok(curr) => (curr, vicinity.center()),
                            Err(curr) => (
                                curr,
                                match vicinity.from(side) {
                                    Some(other) => other,
                                    None => {
                                        sides |= side;
                                        continue;
                                    }
                                },
                            ),
                        };

                        for conn in &connections[level as usize..(level + shape_height) as usize] {
                            if !other.connection(curr).overlaps(
                                conn,
                                side.opposite(),
                                &self.polygons,
                                Axis::X,
                            ) {
                                sides |= side;
                                break;
                            }

                            curr = match curr.to(Side::Up, 1) {
                                Ok(next) => next,
                                Err(next) => {
                                    other = match vicinity.from_upper(side) {
                                        Some(other) => other,
                                        None => {
                                            sides |= side;
                                            break;
                                        }
                                    };
                                    next
                                }
                            };
                        }
                    }

                    sides
                },
                builder,
            );
        }

        let mesh = builder.mesh(ren);
        builder.clear();
        mesh
    }
}

impl Space for ClusterView {
    fn get(&self, pn: Point) -> Pass {
        self.map.get(pn).copied().unwrap_or_else(Pass::empty)
    }

    fn column(&self, pn: Point, height: Height) -> Column<Pass> {
        const HEIGHT: usize = Height::HEIGHT as usize;
        const EMPTY: [Pass; HEIGHT] = [Pass::empty(); HEIGHT];

        self.map
            .column(pn, height)
            .unwrap_or_else(|| Column(&EMPTY[..height.get() as usize], &[]))
    }
}
