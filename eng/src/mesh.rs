use std::{error, fmt};

pub struct Slot(pub Box<[u32]>);

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

pub struct Mesh<V> {
    verts: Vec<V>,
    indxs: Vec<u32>,
    slots: Vec<Slot>,
}

impl<V> Mesh<V> {
    pub fn new(verts: Vec<V>, indxs: Vec<u32>) -> Result<Self, Error> {
        Self::from_slots(verts, indxs, Vec::default())
    }

    pub fn from_slots(verts: Vec<V>, indxs: Vec<u32>, slots: Vec<Slot>) -> Result<Self, Error> {
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
        if let Some(&face_idx) = slots.iter().find_map(|slot| {
            slot.0
                .iter()
                .find(|&face_idx| *face_idx as usize >= faces_len)
        }) {
            return Err(Error::FaceIndex(face_idx));
        }

        Ok(Self::new_unchecked(verts, indxs, slots))
    }

    pub fn new_unchecked(verts: Vec<V>, indxs: Vec<u32>, slots: Vec<Slot>) -> Self {
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

    pub fn slots(&self) -> &[Slot] {
        &self.slots
    }
}
