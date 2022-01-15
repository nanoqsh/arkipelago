use crate::{
    land::{
        polygon::Polygons,
        variant::{self, Variant},
        Factory, Parameters,
    },
    loader::{
        load::{Cached, EventLoad, Load, Sample, SampleLoad, SpriteLoad, ToShape},
        read::ReadJson,
        Error,
    },
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
    Slot(String),
}

impl fmt::Display for VariantError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "variant is empty"),
            Self::Slot(slot) => write!(f, "wrong slot {slot}"),
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
        rotation: Rotation,
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
    pub fn to_variant<S>(
        &self,
        factory: &mut Factory,
        rotation: Rotation,
        man: &mut Polygons,
        st: S,
    ) -> Result<Variant, variant::Error>
    where
        S: Fn(Option<&str>) -> Vec2,
    {
        let factory = RefCell::new(factory);
        let man = RefCell::new(man);
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
                        rotation: info.rotation + rotation,
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
                        .unwrap_or_else(|| {
                            let vec = st(self.sprite.as_deref());
                            vec![vec; mesh.slots().len()].into_boxed_slice()
                        }),
                    height: *height,
                };

                (
                    mesh,
                    info.sample
                        .conn
                        .iter()
                        .copied()
                        .map(|conn| conn.rotated(info.rotation, &mut man.borrow_mut())),
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
                    } => (name, rotation, discard, sprites),
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

pub(crate) struct VariantLoad<'a> {
    pub read: ReadJson,
    pub sprites: Rc<RefCell<Cached<EventLoad<'a, SpriteLoad>>>>,
    pub samples: Rc<RefCell<Cached<EventLoad<'a, SampleLoad<'a>>>>>,
}

impl Load for VariantLoad<'_> {
    type Asset = ToVariant;
    type Error = Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Error> {
        let content = self.read.read(name)?;
        let raw = serde_json::from_str(content)?;
        load(
            raw,
            |name| self.sprites.borrow_mut().load(name),
            |name| self.samples.borrow_mut().load(name),
        )
    }
}
