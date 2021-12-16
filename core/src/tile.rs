use crate::{height::Height, load::load_tiles, path::Pass};
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

pub struct VariantInfo {
    idx: VariantIndex,
    name: String,
    passes: Vec<Pass>,
}

impl VariantInfo {
    pub fn index(&self) -> VariantIndex {
        self.idx
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn passes(&self) -> &[Pass] {
        &self.passes
    }
}

pub struct TileInfo {
    idx: TileIndex,
    name: Rc<str>,
    height: Height,
    variants: Vec<VariantInfo>,
}

impl TileInfo {
    pub fn index(&self) -> TileIndex {
        self.idx
    }

    pub fn name(&self) -> &str {
        &self.name
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
    map: HashMap<Rc<str>, TileIndex>,
    vec: Vec<TileInfo>,
}

impl TileList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut list = Self {
            map: HashMap::default(),
            vec: vec![TileInfo {
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
        V: IntoIterator<Item = (String, Vec<Pass>)>,
    {
        let idx = self.vec.len();
        assert!(idx <= u16::MAX as usize);
        let tile_idx = TileIndex::new(idx as u16).unwrap();
        let name = name.into();
        let old = self.map.insert(Rc::clone(&name), tile_idx);
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
                        assert_eq!(height.get() as usize, passes.len());
                        passes
                    },
                })
                .collect(),
            height,
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
