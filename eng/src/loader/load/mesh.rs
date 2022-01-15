use crate::{
    loader::{load::Load, read::ReadJson, Error},
    mesh::{self, Slots},
    Mesh, Vert,
};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub(crate) struct RawMesh {
    verts: Vec<RawVertex>,
    indxs: Vec<u32>,
    #[serde(default)]
    slots: HashMap<String, Box<[u32]>>,
}

#[derive(Deserialize)]
struct RawVertex {
    c: [f32; 3],
    n: [f32; 3],
    t: [f32; 2],
}

fn load(mesh: RawMesh) -> Result<Mesh, mesh::Error> {
    let RawMesh {
        verts,
        indxs,
        slots,
    } = mesh;

    Mesh::new(
        verts
            .into_iter()
            .map(|raw| Vert {
                co: raw.c.into(),
                nm: raw.n.into(),
                st: raw.t.into(),
            })
            .collect(),
        indxs,
        Slots::new(slots),
    )
}

pub(crate) struct MeshLoad {
    pub read: ReadJson,
}

impl Load for MeshLoad {
    type Asset = Mesh;
    type Error = Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Self::Error> {
        let content = self.read.read(name)?;
        let raw = serde_json::from_str(content)?;
        let mesh = load(raw)?;
        Ok(mesh)
    }
}
