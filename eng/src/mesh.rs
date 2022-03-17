use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    IndxsLen(usize),
    VertIndex(u32),
    FaceIndex(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IndxsLen(len) => write!(f, "wrong indices len {len}"),
            Self::VertIndex(idx) => write!(f, "wrong vertex index {idx}"),
            Self::FaceIndex(idx) => write!(f, "wrong face index {idx}"),
        }
    }
}

impl error::Error for Error {}

#[derive(Default)]
pub struct Slots(Vec<(String, Box<[u32]>)>);

impl Slots {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Box<[u32]>)>,
    {
        Self(iter.into_iter().collect())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn index(&self, key: &str) -> Option<u32> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i as u32)
    }

    pub fn values(&self) -> impl Iterator<Item = (&str, u32)> + '_ {
        self.0
            .iter()
            .enumerate()
            .map(|(i, (k, _))| (k.as_str(), i as u32))
    }

    pub fn ordered_keys(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|(k, _)| k.as_str())
    }

    pub fn for_face(&self, face: u32) -> Option<(&str, u32)> {
        self.0
            .iter()
            .enumerate()
            .find(|(_, (_, slot))| slot.contains(&face))
            .map(|(i, (k, _))| (k.as_str(), i as u32))
    }

    pub fn face_indices(&self) -> impl Iterator<Item = u32> + '_ {
        self.0.iter().flat_map(|(_, slot)| slot.iter().copied())
    }

    pub fn faces_max_len(&self) -> usize {
        self.0.iter().map(|(_, slot)| slot.len()).sum()
    }
}

pub struct Mesh<V> {
    verts: Vec<V>,
    indxs: Vec<u32>,
    slots: Slots,
}

impl<V> Mesh<V> {
    pub fn new(verts: Vec<V>, indxs: Vec<u32>, slots: Slots) -> Result<Self, Error> {
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

    pub fn new_unchecked(verts: Vec<V>, indxs: Vec<u32>, slots: Slots) -> Self {
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

    pub fn slots(&self) -> &Slots {
        &self.slots
    }
}
