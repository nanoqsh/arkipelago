use crate::{
    loader::re::*,
    mesh::{self, Mesh, Slot},
    Vert,
};
use std::collections::HashMap;

pub(crate) struct MeshLoad;

impl<'a> Load<'a> for MeshLoad {
    const PATH: &'static str = "meshes";
    type Format = Json<'a, RawMesh<'a>>;
    type Asset = Mesh<Vert>;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        let mesh = load(raw)?;
        Ok(mesh)
    }
}

#[derive(Deserialize)]
pub(crate) struct RawMesh<'s> {
    verts: Vec<RawVertex>,
    indxs: Vec<u32>,
    #[serde(default, borrow)]
    slots: HashMap<&'s str, Box<[u32]>>,
}

#[derive(Deserialize)]
struct RawVertex {
    c: [f32; 3],
    n: [f32; 3],
    t: [f32; 2],
}

fn load(mesh: RawMesh) -> Result<Mesh<Vert>, mesh::Error> {
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
        slots
            .into_iter()
            .map(|(_, face_indxs)| Slot(face_indxs))
            .collect(),
    )
}
