use crate::{height::Height, load::load_tiles, path::Pass, prelude::Rotation};
use std::{collections::HashMap, fmt, rc::Rc};

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

pub struct Variant {
    pub idx: VariantIndex,
    pub name: String,
    pub rotation: Rotation,
    pub passes: Vec<Pass>,
}

pub struct Tile {
    pub idx: TileIndex,
    pub name: Rc<str>,
    pub height: Height,
    pub variants: Vec<Variant>,
}

impl Tile {
    pub fn variant(&self, idx: VariantIndex) -> &Variant {
        &self.variants[idx.0 as usize]
    }
}

pub struct TileList {
    map: HashMap<Rc<str>, TileIndex>,
    vec: Vec<Tile>,
}

impl TileList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut list = Self {
            map: HashMap::default(),
            vec: vec![Tile {
                idx: TileIndex::new(1).unwrap(),
                name: "".into(),
                height: Height::new(1).unwrap(),
                variants: Vec::default(),
            }],
        };

        load_tiles(&mut list).expect("load tiles");
        list
    }

    pub fn add<V>(&mut self, name: &str, height: Height, variants: V)
    where
        V: IntoIterator<Item = (String, Rotation, Vec<Pass>)>,
    {
        let idx = self.vec.len();
        assert!(idx <= u16::MAX as usize);
        let tile_idx = TileIndex::new(idx as u16).unwrap();
        let name = name.into();
        let old = self.map.insert(Rc::clone(&name), tile_idx);
        assert!(old.is_none());
        self.vec.push(Tile {
            idx: tile_idx,
            name,
            variants: variants
                .into_iter()
                .enumerate()
                .map(|(idx, (name, rotation, passes))| Variant {
                    idx: {
                        assert!(idx <= u8::MAX as usize);
                        VariantIndex(idx as u8)
                    },
                    name,
                    rotation,
                    passes: {
                        assert_eq!(height.get() as usize, passes.len());
                        passes
                    },
                })
                .collect(),
            height,
        });
    }

    pub fn get(&self, idx: TileIndex) -> &Tile {
        &self.vec[idx.0 as usize]
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Tile> {
        let idx = self.map.get(name)?;
        Some(self.get(*idx))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tile> {
        self.vec.iter()
    }
}
