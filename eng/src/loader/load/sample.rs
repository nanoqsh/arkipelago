use crate::{land::Overlay, Mesh};
use core::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc};

#[derive(Deserialize)]
#[serde(untagged)]
enum RawMesh<'a> {
    Name(&'a str),
    Obj {
        name: &'a str,
        #[serde(default)]
        rename: HashMap<&'a str, &'a str>,
        #[serde(default)]
        contact: HashMap<&'a str, Sides>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawOverlay<'a> {
    Tag(&'a str),
    Polygon(Vec<(f32, f32)>),
}

#[derive(Deserialize)]
struct RawSlab<'a> {
    #[serde(borrow)]
    mesh: Option<RawMesh<'a>>,
    #[serde(default)]
    overlay: HashMap<Sides, RawOverlay<'a>>,
}

type RawSample<'a> = Vec<RawSlab<'a>>;

type Slot = u32;

struct ToShape {
    mesh: Rc<Mesh>,
    contact: HashMap<Slot, Sides>,
}

struct Slab {
    shape: Option<ToShape>,
    overlay: [Overlay; 6],
}

pub struct Sample(Box<[Slab]>);
