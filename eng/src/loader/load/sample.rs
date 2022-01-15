use crate::{
    land::{
        polygon::{Polygon, Polygons},
        Connections, Overlay,
    },
    loader::{
        load::{Cached, EventLoad, Load, MeshLoad},
        read::ReadJson,
        Error,
    },
    Mesh,
};
use core::prelude::*;
use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, error, fmt, rc::Rc};

#[derive(Debug)]
enum SampleError {
    OverlayLen(usize),
    Overlay(String),
}

impl fmt::Display for SampleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::OverlayLen(len) => write!(f, "wrong overlay len {}", len),
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
        height: Option<Height>,
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
    pub height: Height,
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
                RawMesh::Name(name) => (name, Height::default(), HashMap::default()),
                RawMesh::Obj {
                    name,
                    height,
                    contact,
                } => (name, height.unwrap_or_default(), contact),
            };

            let overlay_len = sample.overlay.len();
            if overlay_len != height.get() as usize {
                return Err(SampleError::OverlayLen(overlay_len).into());
            }

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

pub(crate) struct SampleLoad<'a> {
    pub read: ReadJson,
    pub meshes: Rc<RefCell<Cached<EventLoad<'a, MeshLoad>>>>,
    pub polygons: Polygons,
}

impl Load for SampleLoad<'_> {
    type Asset = Sample;
    type Error = Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Self::Error> {
        let content = self.read.read(name)?;
        let raw = serde_json::from_str(content)?;
        load(
            raw,
            |name| self.meshes.borrow_mut().load(name),
            &mut self.polygons,
        )
    }
}
