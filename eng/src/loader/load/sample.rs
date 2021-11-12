use crate::{
    land::{
        polygon::{Polygon, Polygons},
        Connections, Overlay,
    },
    loader::{load::MeshLoad, re::*, reader::Reader},
    Mesh,
};
use core::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, error, fmt, rc::Rc};

#[derive(Debug)]
enum SampleError {
    Overlay(String),
}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Overlay(str) => write!(f, "wrong overlay {}", str),
        }
    }
}

impl error::Error for SampleError {}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawMesh<'a> {
    Name(&'a str),
    Obj {
        name: &'a str,
        height: Option<u8>,
        #[serde(default)]
        contact: HashMap<String, Sides>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawOverlay<'a> {
    Tag(&'a str),
    Polygon(Polygon),
}

#[derive(Deserialize)]
pub(crate) struct RawSample<'a> {
    #[serde(borrow)]
    mesh: RawMesh<'a>,
    #[serde(default)]
    overlay: Vec<HashMap<Sides, RawOverlay<'a>>>,
}

pub(crate) struct ToShape {
    pub mesh: Rc<Mesh>,
    pub height: u8,
    pub contact: HashMap<String, Sides>,
}

pub(crate) struct Sample {
    pub shape: ToShape,
    pub conn: Vec<Connections>,
}

fn load<M>(sample: RawSample, mut load_mesh: M, polygons: &mut Polygons) -> Result<Sample, Error>
where
    M: FnMut(&str) -> Result<Rc<Mesh>, Error>,
{
    Ok(Sample {
        shape: {
            let (name, height, contact) = match sample.mesh {
                RawMesh::Name(name) => (name, 1, HashMap::default()),
                RawMesh::Obj {
                    name,
                    height,
                    contact,
                } => (name, height.unwrap_or(1), contact),
            };

            ToShape {
                mesh: load_mesh(name)?,
                height,
                contact,
            }
        },
        conn: sample
            .overlay
            .into_iter()
            .map(|overlay| {
                let mut conn = Connections::new();
                for (sides, raw) in overlay {
                    conn.set(
                        sides,
                        match raw {
                            RawOverlay::Tag(tag) => match tag {
                                "none" => Overlay::new_none(),
                                "full" => Overlay::new_full(),
                                _ => return Err(SampleError::Overlay(tag.into())),
                            },
                            RawOverlay::Polygon(poly) => Overlay::from_polygon(poly, polygons),
                        },
                    )
                }
                Ok(conn)
            })
            .collect::<Result<_, _>>()?,
    })
}

pub(crate) struct SampleLoad<'a, 'b> {
    pub meshes: &'a mut Reader<'b, Mesh, String>,
    pub polygons: &'a mut Polygons,
}

impl<'a> Load<'a> for SampleLoad<'a, '_> {
    const PATH: &'static str = "samples";
    type Format = Json<'a, RawSample<'a>>;
    type Asset = Sample;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        load(
            raw,
            |name| self.meshes.read_json(name, MeshLoad),
            self.polygons,
        )
    }
}
