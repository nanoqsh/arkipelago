use crate::chunk;
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

#[derive(Copy, Clone)]
pub struct Height(u8);

impl Height {
    pub const fn new(height: u8) -> Option<Self> {
        const HEIGHT: u8 = chunk::HEIGHT as u8;

        match height {
            1..=HEIGHT => Some(Self(height)),
            _ => None,
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct TileInfo {
    idx: TileIndex,
    name: &'static str,
    height: Height,
    variants: Vec<(&'static str, VariantIndex)>,
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

    pub fn variants(&self) -> &[(&'static str, VariantIndex)] {
        &self.variants
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
                "dirt",
                "dirt_bevel_q0",
                "dirt_bevel_q1",
                "dirt_bevel_q2",
                "dirt_bevel_q3",
            ],
        );
        list.add("grass", 2, ["grass_0", "grass_1"]);
        list.add("rocks", 1, ["rocks"]);
        list.add(
            "stone",
            2,
            [
                "stone",
                "stone_bevel_q0",
                "stone_bevel_q1",
                "stone_bevel_q2",
                "stone_bevel_q3",
                "stone_bevel_vertical_q0",
                "stone_bevel_vertical_q1",
                "stone_bevel_vertical_q2",
                "stone_bevel_vertical_q3",
            ],
        );
        list.add("bricks", 2, ["bricks"]);
        list.add("box", 2, ["box"]);
        list.add("steps", 2, ["steps_q0", "steps_q1", "steps_q2", "steps_q3"]);
        list
    }

    pub fn add<V>(&mut self, name: &'static str, height: u8, variants: V)
    where
        V: IntoIterator<Item = &'static str>,
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
                .map(|(idx, name)| {
                    assert!(idx <= u8::MAX as usize);
                    (name, VariantIndex(idx as u8))
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
