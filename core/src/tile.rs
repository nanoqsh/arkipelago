use crate::{height::Height, path::Pass, rotation::Rotation};
use std::{collections::HashMap, fmt};

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct TileIndex(u16);

impl TileIndex {
    pub const fn new(idx: u16) -> Option<Self> {
        match idx {
            0 => None,
            _ => Some(Self(idx)),
        }
    }

    pub const fn get(self) -> u16 {
        self.0
    }
}

impl fmt::Display for TileIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tile: {}", self.0)
    }
}

impl fmt::Debug for TileIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct VariantIndex(pub u8);

impl VariantIndex {
    pub const fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for VariantIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "variant: {}", self.0)
    }
}

impl fmt::Debug for VariantIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct VariantInfo {
    idx: VariantIndex,
    name: &'static str,
    passes: Vec<Pass>,
}

impl VariantInfo {
    pub fn index(&self) -> VariantIndex {
        self.idx
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn passes(&self) -> &[Pass] {
        &self.passes
    }
}

pub struct TileInfo {
    idx: TileIndex,
    name: &'static str,
    height: Height,
    variants: Vec<VariantInfo>,
}

impl TileInfo {
    pub fn index(&self) -> TileIndex {
        self.idx
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn variants(&self) -> &[VariantInfo] {
        &self.variants
    }

    pub fn variant(&self, idx: VariantIndex) -> &VariantInfo {
        &self.variants[idx.0 as usize]
    }
}

pub struct TileList {
    map: HashMap<&'static str, TileIndex>,
    vec: Vec<TileInfo>,
}

impl TileList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut list = Self {
            map: HashMap::default(),
            vec: vec![TileInfo {
                idx: TileIndex::new(1).unwrap(),
                name: "",
                height: Height::new(1).unwrap(),
                variants: Vec::default(),
            }],
        };

        list.add(
            "dirt",
            1,
            [
                ("dirt", vec![Pass::solid()]),
                ("dirt_bevel_q0", vec![Pass::ascent([Rotation::Q0])]),
                ("dirt_bevel_q1", vec![Pass::ascent([Rotation::Q1])]),
                ("dirt_bevel_q2", vec![Pass::ascent([Rotation::Q2])]),
                ("dirt_bevel_q3", vec![Pass::ascent([Rotation::Q3])]),
            ],
        );
        list.add(
            "grass",
            2,
            [
                ("grass_0", vec![Pass::empty(), Pass::empty()]),
                ("grass_1", vec![Pass::empty(), Pass::empty()]),
            ],
        );
        list.add("rocks", 1, [("rocks", vec![Pass::empty()])]);
        list.add(
            "stone",
            2,
            [
                ("stone", vec![Pass::solid(), Pass::solid()]),
                ("stone_bevel_q0", vec![Pass::solid(), Pass::solid()]),
                ("stone_bevel_q1", vec![Pass::solid(), Pass::solid()]),
                ("stone_bevel_q2", vec![Pass::solid(), Pass::solid()]),
                ("stone_bevel_q3", vec![Pass::solid(), Pass::solid()]),
                (
                    "stone_bevel_vertical_q0",
                    vec![Pass::solid(), Pass::pathless()],
                ),
                (
                    "stone_bevel_vertical_q1",
                    vec![Pass::solid(), Pass::pathless()],
                ),
                (
                    "stone_bevel_vertical_q2",
                    vec![Pass::solid(), Pass::pathless()],
                ),
                (
                    "stone_bevel_vertical_q3",
                    vec![Pass::solid(), Pass::pathless()],
                ),
            ],
        );
        list.add(
            "bricks",
            2,
            [("bricks", vec![Pass::solid(), Pass::solid()])],
        );
        list.add("box", 2, [("box", vec![Pass::solid(), Pass::solid()])]);
        list.add(
            "steps",
            2,
            [
                (
                    "steps_q0",
                    vec![Pass::solid(), Pass::ascent([Rotation::Q0])],
                ),
                (
                    "steps_q1",
                    vec![Pass::solid(), Pass::ascent([Rotation::Q1])],
                ),
                (
                    "steps_q2",
                    vec![Pass::solid(), Pass::ascent([Rotation::Q2])],
                ),
                (
                    "steps_q3",
                    vec![Pass::solid(), Pass::ascent([Rotation::Q3])],
                ),
            ],
        );
        list
    }

    pub fn add<V>(&mut self, name: &'static str, height: u8, variants: V)
    where
        V: IntoIterator<Item = (&'static str, Vec<Pass>)>,
    {
        let idx = self.vec.len();
        assert!(idx <= u16::MAX as usize);
        let tile_idx = TileIndex::new(idx as u16).unwrap();
        let old = self.map.insert(name, tile_idx);
        assert!(old.is_none());
        self.vec.push(TileInfo {
            idx: tile_idx,
            name,
            variants: variants
                .into_iter()
                .enumerate()
                .map(|(idx, (name, passes))| VariantInfo {
                    idx: {
                        assert!(idx <= u8::MAX as usize);
                        VariantIndex(idx as u8)
                    },
                    name,
                    passes: {
                        assert_eq!(passes.len(), height as usize);
                        passes
                    },
                })
                .collect(),
            height: Height::new(height).unwrap(),
        });
    }

    pub fn get(&self, idx: TileIndex) -> &TileInfo {
        self.vec.get(idx.0 as usize).unwrap()
    }

    pub fn get_by_name(&self, name: &str) -> Option<&TileInfo> {
        let idx = self.map.get(name)?;
        Some(self.get(*idx))
    }

    pub fn iter(&self) -> impl Iterator<Item = &TileInfo> {
        self.vec.iter()
    }
}
