use crate::{
    land::{
        variant::{self, Slab, Variant},
        Factory, Parameters,
    },
    loader::{
        load::{Sample, SampleLoad, SpriteLoad},
        re::*,
        reader::Reader,
    },
    Mesh,
};
use core::prelude::*;
use image::DynamicImage;
use serde::Deserialize;
use shr::cgm::Vec2;
use std::{
    cell::RefCell,
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

impl ToVariant {
    pub fn to_variant<S>(&self, factory: &mut Factory, st: S) -> Result<Variant, variant::Error>
    where
        S: Fn(Option<&str>) -> Vec2,
    {
        let factory = RefCell::new(factory);
        Variant::new(
            self.samples.iter().flat_map(|info| {
                info.sample.shapes().map(|shape| match shape {
                    None => Slab::Empty,
                    Some(shape) => Slab::Mesh {
                        shape: factory.borrow_mut().make(Parameters {
                            mesh: &shape.mesh,
                            rotation: info.rotation,
                            discard: &info.discard,
                            contact: &shape.contact,
                        }),
                        sprites_st: info
                            .sprites
                            .as_ref()
                            .map(|sprites| match sprites {
                                Sprites::One(sprite) => {
                                    let st = st(Some(sprite));
                                    std::iter::repeat(st)
                                        .take(shape.mesh.slots().len())
                                        .collect()
                                }
                                Sprites::Many(map) => shape
                                    .mesh
                                    .slots()
                                    .ordered_keys()
                                    .map(|key| st(map.get(key).map(String::as_str)))
                                    .collect(),
                            })
                            .unwrap_or_default(),
                    },
                })
            }),
            st(self.sprite.as_deref()),
        )
    }
}

fn load<S, T>(
    variant: RawVariant,
    mut load_sprite: S,
    mut load_sample: T,
) -> Result<ToVariant, Error>
where
    S: FnMut(&str) -> Result<Rc<DynamicImage>, Error>,
    T: FnMut(&str) -> Result<Rc<Sample>, Error>,
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

            if let Some(sprites) = &sprites {
                match sprites {
                    Sprites::One(sprite) => {
                        let _ = load_sprite(sprite);
                    }
                    Sprites::Many(map) => {
                        for sprite in map.values() {
                            let _ = load_sprite(sprite);
                        }
                    }
                }
            }

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

    if let Some(sprite) = &variant.sprite {
        let _ = load_sprite(sprite);
    }

    Ok(ToVariant {
        samples: samples?,
        sprite: variant.sprite,
    })
}

pub(crate) struct VariantLoad<'a, 'b> {
    pub sprites: &'a mut Reader<'b, DynamicImage>,
    pub meshes: &'a mut Reader<'b, Mesh, String>,
    pub samples: &'a mut Reader<'b, Sample, String>,
}

impl<'a> Load<'a> for VariantLoad<'a, '_> {
    const PATH: &'static str = "variants";
    type Format = Json<'a, RawVariant<'a>>;
    type Asset = ToVariant;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        const PREFIX: &str = "tiles/";
        let mut prefix = String::with_capacity(16);

        load(
            raw,
            |name| {
                prefix.clear();
                prefix.push_str(PREFIX);
                prefix.push_str(name);
                self.sprites.read_png(&prefix, SpriteLoad)
            },
            |name| {
                self.samples.read_json(
                    name,
                    SampleLoad {
                        meshes: self.meshes,
                    },
                )
            },
        )
    }
}