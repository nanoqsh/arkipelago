use crate::{
    land::{
        polygon::Polygons,
        variant::{self, Variant},
        Factory, Parameters,
    },
    loader::{
        load::{Sample, SampleLoad, SpriteLoad, ToShape},
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
            self.samples.iter().map(|info| {
                let ToShape {
                    mesh,
                    height,
                    contact,
                } = &info.sample.shape;

                let mesh = variant::Mesh {
                    shape: factory.borrow_mut().make(Parameters {
                        mesh,
                        rotation: info.rotation,
                        discard: &info.discard,
                        contact,
                    }),
                    sprites_st: info
                        .sprites
                        .as_ref()
                        .map(|sprites| match sprites {
                            Sprites::One(sprite) => std::iter::repeat(st(Some(sprite)))
                                .take(mesh.slots().len())
                                .collect(),
                            Sprites::Many(map) => mesh
                                .slots()
                                .ordered_keys()
                                .map(|slot| match map.get(slot) {
                                    None => st(self.sprite.as_deref()),
                                    Some(name) => st(Some(name)),
                                })
                                .collect(),
                        })
                        .unwrap_or_default(),
                    height: *height,
                };

                (
                    mesh,
                    info.sample
                        .conn
                        .iter()
                        .copied()
                        .map(|conn| conn.rotated(info.rotation)),
                )
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

    if let Some(sprite) = &variant.sprite {
        let _ = load_sprite(sprite);
    }

    Ok(ToVariant {
        samples: variant
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
                        Rotation::from_quarters(rotation)
                            .ok_or(VariantError::Rotation(rotation))?,
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

                let sample = load_sample(name)?;
                for slot in &discard {
                    if sample.shape.mesh.slots().index(slot).is_none() {
                        return Err(VariantError::Slot(slot.into()).into());
                    }
                }

                Ok(SampleInfo {
                    sample,
                    rotation,
                    discard,
                    sprites,
                })
            })
            .collect::<Result<_, _>>()?,
        sprite: variant.sprite,
    })
}

pub(crate) struct VariantLoad<'a, 'b> {
    pub sprites: &'a mut Reader<'b, DynamicImage>,
    pub meshes: &'a mut Reader<'b, Mesh, String>,
    pub samples: &'a mut Reader<'b, Sample, String>,
    pub polygons: &'a mut Polygons,
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
                        polygons: self.polygons,
                    },
                )
            },
        )
    }
}
