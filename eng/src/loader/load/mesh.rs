use crate::{
    loader::re::*,
    mesh::{self, Slots},
    Mesh, Vert,
};
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

    Mesh::from_slots(
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

pub(crate) struct MeshLoad;

impl<'a> Load<'a> for MeshLoad {
    const PATH: &'static str = "meshes";
    type Format = Json<'a, RawMesh>;
    type Asset = Mesh;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        let mesh = load(raw)?;
        Ok(mesh)
    }
}
