use std::{collections::HashMap, error, fmt};

#[derive(Debug)]
pub enum Error {
    IndxsLen(usize),
    VertIndex(u32),
    FaceIndex(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndxsLen(len) => write!(f, "wrong indices len {}", len),
            Self::VertIndex(idx) => write!(f, "wrong vertex index {}", idx),
            Self::FaceIndex(idx) => write!(f, "wrong face index {}", idx),
        }
    }
}

impl error::Error for Error {}

pub trait Key {
    type Keys;
}

impl Key for u32 {
    type Keys = ();
}

impl Key for str {
    type Keys = HashMap<String, u32>;
}

pub struct Slots<K>
where
    K: Key + ?Sized,
{
    keys: K::Keys,
    slots: Vec<Box<[u32]>>,
}

impl<K> Slots<K>
where
    K: Key + ?Sized,
{
    pub fn faces(&self, idx: u32) -> Option<&[u32]> {
        self.slots.get(idx as usize).map(|slot| &slot[..])
    }

    pub fn face_indices(&self) -> impl Iterator<Item = u32> + '_ {
        self.slots.iter().map(|slot| slot.iter().copied()).flatten()
    }

    pub fn faces_max_len(&self) -> usize {
        self.slots.iter().map(|slot| slot.len()).sum()
    }
}

impl Slots<str> {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Box<[u32]>)>,
    {
        let (keys, slots) = iter
            .into_iter()
            .enumerate()
            .map(|(idx, (key, slot))| ((key, idx as u32), slot))
            .unzip();

        Self { keys, slots }
    }

    pub fn index(&self, key: &str) -> Option<u32> {
        self.keys.get(key).copied()
    }

    pub fn values(&self) -> impl Iterator<Item = (&str, u32)> + '_ {
        self.keys.iter().map(|(k, v)| (k.as_str(), *v))
    }

    pub fn ordered_keys(&self) -> impl Iterator<Item = &str> {
        self.slots.iter().enumerate().map(|(target, _)| {
            self.keys
                .iter()
                .find(|(_, &idx)| idx == target as u32)
                .unwrap()
                .0
                .as_str()
        })
    }

    pub fn for_face(&self, face: u32) -> Option<(&str, u32)> {
        self.values()
            .find(|(_, slot_idx)| self[*slot_idx as usize].contains(&face))
    }
}

impl<K> Default for Slots<K>
where
    K: Key + ?Sized,
    K::Keys: Default,
{
    fn default() -> Self {
        Self {
            keys: Default::default(),
            slots: Vec::default(),
        }
    }
}

impl<K> std::ops::Deref for Slots<K>
where
    K: Key + ?Sized,
{
    type Target = [Box<[u32]>];

    fn deref(&self) -> &Self::Target {
        &self.slots
    }
}

pub struct Mesh<V, K>
where
    K: Key + ?Sized,
{
    verts: Vec<V>,
    indxs: Vec<u32>,
    slots: Slots<K>,
}

impl<V, K> Mesh<V, K>
where
    K: Key + ?Sized,
{
    pub fn new(verts: Vec<V>, indxs: Vec<u32>) -> Result<Self, Error>
    where
        K::Keys: Default,
    {
        Self::from_slots(verts, indxs, Slots::default())
    }

    pub fn from_slots(verts: Vec<V>, indxs: Vec<u32>, slots: Slots<K>) -> Result<Self, Error> {
        // Vertices len must be a multiple of 3
        if indxs.len() % 3 != 0 {
            return Err(Error::IndxsLen(indxs.len()));
        }

        // Check each index points to vertex
        if let Some(&idx) = indxs.iter().find(|&idx| *idx as usize >= verts.len()) {
            return Err(Error::VertIndex(idx));
        }

        // Check each slot points to face
        let faces_len = indxs.len() / 3;
        if let Some(face_idx) = slots
            .face_indices()
            .find(|&face_idx| face_idx as usize >= faces_len)
        {
            return Err(Error::FaceIndex(face_idx));
        }

        Ok(Self::new_unchecked(verts, indxs, slots))
    }

    pub fn new_unchecked(verts: Vec<V>, indxs: Vec<u32>, slots: Slots<K>) -> Self {
        Self {
            verts,
            indxs,
            slots,
        }
    }

    pub fn verts(&self) -> &[V] {
        &self.verts
    }

    pub fn indxs(&self) -> &[u32] {
        &self.indxs
    }

    pub fn slots(&self) -> &Slots<K> {
        &self.slots
    }
}

impl<V> From<Mesh<V, str>> for Mesh<V, u32> {
    fn from(mesh: Mesh<V, str>) -> Self {
        Self {
            verts: mesh.verts,
            indxs: mesh.indxs,
            slots: Slots {
                keys: (),
                slots: mesh.slots.slots,
            },
        }
    }
}
