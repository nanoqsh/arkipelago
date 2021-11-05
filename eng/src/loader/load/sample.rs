use crate::{
    land::{polygon::Polygon, Overlay},
    loader::{load::MeshLoad, re::*, reader::Reader},
    Mesh,
};
use core::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, fmt, rc::Rc};

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

impl std::error::Error for SampleError {}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawMesh<'a> {
    Name(&'a str),
    Obj {
        name: &'a str,
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
pub(crate) struct RawSlab<'a> {
    #[serde(borrow)]
    mesh: Option<RawMesh<'a>>,
    #[serde(default)]
    overlay: HashMap<Sides, RawOverlay<'a>>,
}

type RawSample<'a> = Vec<RawSlab<'a>>;

struct ToShape {
    mesh: Rc<Mesh>,
    contact: HashMap<String, Sides>,
}

struct Slab {
    shape: Option<ToShape>,
    overlay: [Overlay; 6],
}

pub struct Sample(Box<[Slab]>);

impl Sample {
    pub fn meshes(&self) -> impl Iterator<Item = &Mesh> + '_ {
        self.0
            .iter()
            .filter_map(|slab| slab.shape.as_ref().map(|shape| Rc::as_ref(&shape.mesh)))
    }
}

fn load<M>(sample: RawSample, mut load_mesh: M) -> Result<Sample, Error>
where
    M: FnMut(&str) -> Result<Rc<Mesh>, Error>,
{
    let slabs: Result<_, _> = sample
        .into_iter()
        .map(|slab| -> Result<_, Error> {
            let shape = slab
                .mesh
                .map(|mesh| {
                    let (name, contact) = match mesh {
                        RawMesh::Name(name) => (name, HashMap::default()),
                        RawMesh::Obj { name, contact } => (name, contact),
                    };

                    load_mesh(name).map(|mesh| ToShape { mesh, contact })
                })
                .transpose()?;

            let mut overlay = [Overlay::default(); 6];
            for (sides, raw) in slab.overlay {
                let raw: RawOverlay = raw;
                let over = match raw {
                    RawOverlay::Tag(tag) => match tag {
                        "none" => Overlay::None,
                        "full" => Overlay::Full,
                        _ => return Err(SampleError::Overlay(tag.into()).into()),
                    },
                    RawOverlay::Polygon(poly) => Overlay::Polygon(Box::leak(Box::new(poly))),
                };

                for side in sides {
                    overlay[side as usize] = over;
                }
            }

            Ok(Slab { shape, overlay })
        })
        .collect();

    Ok(Sample(slabs?))
}

pub(crate) struct SampleLoad<'a, 'b> {
    pub meshes: &'a mut Reader<'b, Mesh, String>,
}

impl<'a> Load<'a> for SampleLoad<'a, '_> {
    const PATH: &'static str = "samples";
    type Format = Json<'a, RawSample<'a>>;
    type Asset = Sample;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        load(raw, |name| self.meshes.read_json(name, MeshLoad))
    }
}
