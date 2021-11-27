use crate::{
    land::{polygon::Polygons, variant::VariantSet, Builder, Connections},
    IndexedMesh, Render,
};
use core::{map::Map, prelude::*};
use shr::cgm::Vec3;
use std::rc::Rc;

pub(crate) struct ClusterView {
    cluster: Cluster,
    local: Map<Chunk<Connections>>,
    variant_set: VariantSet,
    polygons: Polygons,
    builder: Builder,
}

impl ClusterView {
    pub fn new(tile_set: TileSet, variant_set: VariantSet, polygons: Polygons) -> Self {
        Self {
            cluster: Cluster::new(Rc::new(tile_set)),
            local: Map::default(),
            variant_set,
            polygons,
            builder: Builder::with_capacity(64),
        }
    }

    pub fn place(&mut self, gl: GlobalPoint, tile: TileIndex) -> Option<Placed> {
        let placed = self.cluster.place(gl, tile)?;
        let variant = self.variant_set.get((tile, placed.variant));
        let (lo, hi) = self.local.slice_mut(gl, placed.height);
        for (dst, src) in lo.iter_mut().chain(hi).zip(variant.connections()) {
            *dst = *src;
        }

        Some(placed)
    }

    pub fn mesh(&mut self, ren: &Render, offset: Vec3, cl: ClusterPoint) -> IndexedMesh {
        let builder = &mut self.builder;
        for (slice, gl) in self.cluster.tiles(cl).unwrap() {
            let ch = gl.chunk_point();
            let local_offset = {
                let mut v: Vec3 = ch.into();
                v.y *= 0.5;
                v
            };

            let cl = gl.cluster_point();
            let mut vicinity = self.local.vicinity(cl).unwrap();
            let variant = self.variant_set.get(slice.index());
            let variant_height = variant.height();
            let connections = variant.connections();

            variant.build(
                offset + local_offset,
                |level, shape_height| {
                    let mut sides = Sides::empty();

                    if level + shape_height == variant_height {
                        match ch.to(Side::Up, level + shape_height) {
                            Ok(hi) => {
                                let other = vicinity.center().get(hi);
                                if !other.overlaps(
                                    connections.last().unwrap(),
                                    Side::Down,
                                    &self.polygons,
                                ) {
                                    sides |= Side::Up;
                                }
                            }
                            Err(hi) => match vicinity.from(Side::Up) {
                                Some(from) => {
                                    let other = from.get(hi);
                                    if !other.overlaps(
                                        connections.last().unwrap(),
                                        Side::Down,
                                        &self.polygons,
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
                                let other = vicinity.center().get(lo);
                                if !other.overlaps(
                                    connections.first().unwrap(),
                                    Side::Up,
                                    &self.polygons,
                                ) {
                                    sides |= Side::Down;
                                }
                            }
                            Err(lo) => match vicinity.from(Side::Down) {
                                Some(from) => {
                                    let other = from.get(lo);
                                    if !other.overlaps(
                                        connections.first().unwrap(),
                                        Side::Up,
                                        &self.polygons,
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

                        for conn in connections.iter().take(shape_height as usize) {
                            if !other
                                .get(curr)
                                .overlaps(conn, side.opposite(), &self.polygons)
                            {
                                sides |= side;
                            }

                            curr = match curr.to(Side::Up, 1) {
                                Ok(next) => next,
                                Err(next) => {
                                    other = match vicinity.from_upper(side) {
                                        Some(other) => other,
                                        None => {
                                            sides |= side;
                                            continue;
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
