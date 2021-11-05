use crate::{
    loader::{
        load::{Sample, SampleLoad},
        re::*,
        reader::Reader,
    },
    Mesh,
};
use core::prelude::*;
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    error, fmt,
    rc::Rc,
};

#[derive(Debug)]
enum VariantError {
    Empty,
    Rotation(u8),
    Slot(String),
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "variant is empty"),
            Self::Rotation(rotation) => write!(f, "wrong rotation {}", rotation),
            Self::Slot(slot) => write!(f, "wrong slot {}", slot),
        }
    }
}

impl error::Error for VariantError {}

#[derive(Deserialize)]
#[serde(untagged)]
enum Sprites {
    One(String),
    Many(HashMap<String, String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawSample<'a> {
    Name(&'a str),
    Obj {
        name: &'a str,
        #[serde(default)]
        rotation: u8,
        #[serde(default)]
        discard: HashSet<String>,
        sprites: Option<Sprites>,
    },
}

#[derive(Deserialize)]
pub(crate) struct RawVariant<'a> {
    #[serde(borrow)]
    samples: Vec<RawSample<'a>>,
    sprite: Option<String>,
}

struct SampleInfo {
    sample: Rc<Sample>,
    rotation: Rotation,
    discard: HashSet<String>,
    sprites: Option<Sprites>,
}

pub(crate) struct ToVariant {
    samples: Vec<SampleInfo>,
    sprite: Option<String>,
}

fn load<S>(variant: RawVariant, mut load_sample: S) -> Result<ToVariant, Error>
where
    S: FnMut(&str) -> Result<Rc<Sample>, Error>,
{
    if variant.samples.is_empty() {
        return Err(VariantError::Empty.into());
    }

    let samples: Result<_, _> = variant
        .samples
        .into_iter()
        .map(|sample| -> Result<_, Error> {
            let (name, rotation, discard, sprites) = match sample {
                RawSample::Name(name) => (name, Rotation::default(), HashSet::default(), None),
                RawSample::Obj {
                    name,
                    rotation,
                    discard,
                    sprites,
                } => (
                    name,
                    Rotation::from_quarters(rotation).ok_or(VariantError::Rotation(rotation))?,
                    discard,
                    sprites,
                ),
            };

            load_sample(name).and_then(|sample| {
                for slot in &discard {
                    sample
                        .meshes()
                        .find_map(|mesh| mesh.slots().index(slot))
                        .ok_or_else(|| VariantError::Slot(slot.into()))?;
                }

                Ok(SampleInfo {
                    sample,
                    rotation,
                    discard,
                    sprites,
                })
            })
        })
        .collect();

    Ok(ToVariant {
        samples: samples?,
        sprite: variant.sprite,
    })
}

pub(crate) struct VariantLoad<'a, 'b> {
    pub meshes: &'a mut Reader<'b, Mesh, String>,
    pub samples: &'a mut Reader<'b, Sample, String>,
}

impl<'a> Load<'a> for VariantLoad<'a, '_> {
    const PATH: &'static str = "variants";
    type Format = Json<'a, RawVariant<'a>>;
    type Asset = ToVariant;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        load(raw, |name| {
            self.samples.read_json(
                name,
                SampleLoad {
                    meshes: self.meshes,
                },
            )
        })
    }
}
