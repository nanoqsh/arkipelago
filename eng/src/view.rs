use crate::{
    land::{polygon::Polygons, variant::VariantSet, Builder, Connections},
    IndexedMesh, Render,
};
use core::prelude::*;
use shr::cgm::Vec3;
use std::{collections::HashMap, rc::Rc};

pub(crate) struct ClusterView {
    cluster: Cluster,
    local: HashMap<ClusterPoint, Chunk<Connections>>,
    variant_set: VariantSet,
    polygons: Polygons,
    builder: Builder,
}

impl ClusterView {
    pub fn new(tile_set: TileSet, variant_set: VariantSet, polygons: Polygons) -> Self {
        Self {
            cluster: Cluster::new(Rc::new(tile_set)),
            local: HashMap::default(),
            variant_set,
            polygons,
            builder: Builder::with_capacity(64),
        }
    }

    pub fn place(&mut self, gl: GlobalPoint, tile: TileIndex) -> Option<Placed> {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let placed = self.cluster.place(gl, tile)?;
        let variant = self.variant_set.get((tile, placed.variant));

        let mut chunk = self
            .local
            .entry(cl)
            .or_insert_with(|| Chunk::filled(Connections::new()));

        let mut curr = ch;
        for level in 0..placed.height {
            curr = match curr.to(Side::Up, 1) {
                Ok(ch) => ch,
                Err(ch) => {
                    chunk = self
                        .local
                        .entry(cl)
                        .or_insert_with(|| Chunk::filled(Connections::new()));

                    ch
                }
            };

            *chunk.get_mut(curr) = variant.connections()[level as usize];
        }

        Some(placed)
    }

    pub fn mesh(&mut self, ren: &Render, offset: Vec3, cl: ClusterPoint) -> IndexedMesh {
        let builder = &mut self.builder;
        for (slice, gl) in self.cluster.tiles(cl).unwrap() {
            let ch = gl.chunk_point();
            let cl = gl.cluster_point();
            let chunk = self.local.get(&cl).unwrap();
            let variant = self.variant_set.get(slice.index());
            variant.build(
                offset,
                |level, shape_height| {
                    let variant_height = variant.height();
                    let mut sides = Sides::empty();

                    if level + shape_height == variant_height {
                        sides |= Side::Up;
                    } else {
                        match ch.to(Side::Up, level + shape_height) {
                            Ok(hi) => {
                                // let other = this_chunk.connection_at(hi);
                                todo!()
                            }
                            Err(hi) => {
                                // let other = upper_chunk.connection_at(hi);
                                todo!()
                            }
                        }
                    }

                    if level == 0 {
                        sides |= Side::Down;
                    } else {
                        match ch.to(Side::Down, 1) {
                            Ok(lo) => todo!(),
                            Err(lo) => todo!(),
                        }
                    }

                    for side in [Side::Left, Side::Right, Side::Forth, Side::Back] {
                        let (mut curr, _) = match ch.to(side, 1) {
                            Ok(curr) => (curr, ()),
                            Err(curr) => (curr, ()),
                        };

                        for _ in 0..shape_height {
                            let (next, _) = match curr.to(Side::Up, 1) {
                                Ok(next) => (next, ()),
                                Err(next) => (next, ()),
                            };
                            curr = next;
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
