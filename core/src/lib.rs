pub mod chunk_point;
pub mod rotation;
pub mod side;

pub mod prelude {
    pub use crate::{
        chunk_point::ChunkPoint,
        rotation::Rotation,
        side::{Side, Sides},
    };
}

const CHUNK_SIDE: usize = 16;
const CHUNK_HEIGHT: usize = CHUNK_SIDE * 2;
