use crate::{land::vec_map::Map, IndexedMesh, Render, Vert};

pub(crate) struct VertexData {
    pub face: [u32; 3],
    pub slot: u32,
}

pub(crate) struct Builder {
    verts: Vec<Vert>,
    indxs: Vec<u32>,
    added: Map<u32>,
}

impl Builder {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            verts: Vec::with_capacity(cap),
            indxs: Vec::with_capacity(cap),
            added: Map::with_capacity(16),
        }
    }

    pub fn extend<D, V>(&mut self, data: D, vertex: V)
    where
        D: IntoIterator<Item = VertexData>,
        V: Fn(u32, u32) -> Vert,
    {
        unsafe {
            let indxs_offset = self.indxs.len();
            let verts_offset = self.verts.len() as u32;

            let mut counter = 0;
            self.added.clear();
            for vd in data {
                self.indxs.extend(vd.face);
                for idx in vd.face {
                    if self.added.insert(idx, counter) {
                        counter += 1;
                        self.verts.push(vertex(idx, vd.slot));
                    }
                }
            }

            for idx in self.indxs[indxs_offset..].iter_mut() {
                *idx = self.added.get_unchecked(*idx) + verts_offset;
            }
        }
    }

    pub fn mesh(&self, ren: &Render) -> IndexedMesh {
        ren.make_mesh(&self.verts, &self.indxs)
    }

    pub fn clear(&mut self) {
        self.verts.clear();
        self.indxs.clear();
        self.added.clear();
    }
}
