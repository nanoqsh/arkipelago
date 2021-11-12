use crate::{side::*, CHUNK_HEIGHT, CHUNK_SIDE};
use shr::cgm::UVec3;
use std::fmt;

const CHUNK_SIDE_MAX: u8 = CHUNK_SIDE as u8 - 1;
const CHUNK_HEIGHT_MAX: u8 = CHUNK_HEIGHT as u8 - 1;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct ChunkPoint(u16);

impl ChunkPoint {
    pub fn new(x: u8, y: u8, z: u8) -> Option<Self> {
        let (x, y, z) = match (x, y, z) {
            (0..=CHUNK_SIDE_MAX, 0..=CHUNK_HEIGHT_MAX, 0..=CHUNK_SIDE_MAX) => (x, y, z),
            _ => return None,
        };

        Some(Self(x as u16 | (y as u16) << 4 | (z as u16) << 9))
    }

    pub fn points(self) -> (u8, u8, u8) {
        (
            (self.0 & 0b1111) as u8,
            (self.0 >> 4 & 0b11111) as u8,
            (self.0 >> 9 & 0b1111) as u8,
        )
    }

    pub fn x(self) -> u8 {
        self.points().0
    }

    pub fn y(self) -> u8 {
        self.points().1
    }

    pub fn z(self) -> u8 {
        self.points().2
    }

    /// Returns the point moved to `side` by `n`.
    /// If `Ok` returns, then the point is in this chunk,
    /// `Err` in the neighboring one.
    pub fn to(self, side: Side, n: u8) -> Result<Self, Self> {
        let (x, y, z) = self.points();
        match side {
            Side::Left => Self::new(x.saturating_add(n), y, z)
                .ok_or_else(|| Self::new(x.saturating_add(n) - CHUNK_SIDE as u8, y, z).unwrap()),
            Side::Right => {
                if x >= n {
                    Ok(Self::new(x - n, y, z).unwrap())
                } else {
                    Err(Self::new((CHUNK_SIDE as u8).wrapping_sub(n) + x, y, z).unwrap())
                }
            }
            Side::Up => Self::new(x, y.saturating_add(n), z)
                .ok_or_else(|| Self::new(x, y.saturating_add(n) - CHUNK_HEIGHT as u8, z).unwrap()),
            Side::Down => {
                if y >= n {
                    Ok(Self::new(x, y - n, z).unwrap())
                } else {
                    Err(Self::new(x, (CHUNK_HEIGHT as u8).wrapping_sub(n) + y, z).unwrap())
                }
            }
            Side::Forth => Self::new(x, y, z.saturating_add(n))
                .ok_or_else(|| Self::new(x, y, z.saturating_add(n) - CHUNK_SIDE as u8).unwrap()),
            Side::Back => {
                if z >= n {
                    Ok(Self::new(x, y, z - n).unwrap())
                } else {
                    Err(Self::new(x, y, (CHUNK_SIDE as u8).wrapping_sub(n) + z).unwrap())
                }
            }
        }
    }
}

impl TryFrom<UVec3> for ChunkPoint {
    type Error = ();

    fn try_from(vec: UVec3) -> Result<Self, Self::Error> {
        let (x, y, z) = vec.into();
        if x >= CHUNK_SIDE as u32 || y >= CHUNK_HEIGHT as u32 || z >= CHUNK_SIDE as u32 {
            return Err(());
        }

        Self::new(x as u8, y as u8, z as u8).ok_or(())
    }
}

impl fmt::Display for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.points();
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

impl fmt::Debug for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_left() {
        let a = ChunkPoint::new(1, 0, 0).unwrap();
        assert_eq!(a.to(Side::Left, 0), Ok(a));
        assert_eq!(a.to(Side::Left, 1), Ok(ChunkPoint::new(2, 0, 0).unwrap()));
        assert_eq!(a.to(Side::Left, 15), Err(ChunkPoint::new(0, 0, 0).unwrap()));
    }

    #[test]
    fn to_right() {
        let a = ChunkPoint::new(1, 0, 0).unwrap();
        assert_eq!(a.to(Side::Right, 1), Ok(ChunkPoint::new(0, 0, 0).unwrap()));
        assert_eq!(
            a.to(Side::Right, 2),
            Err(ChunkPoint::new(15, 0, 0).unwrap())
        );
    }

    #[test]
    fn to_up() {
        let a = ChunkPoint::new(0, 1, 0).unwrap();
        assert_eq!(a.to(Side::Up, 0), Ok(a));
        assert_eq!(a.to(Side::Up, 1), Ok(ChunkPoint::new(0, 2, 0).unwrap()));
        assert_eq!(a.to(Side::Up, 31), Err(ChunkPoint::new(0, 0, 0).unwrap()));
    }

    #[test]
    fn to_down() {
        let a = ChunkPoint::new(0, 1, 0).unwrap();
        assert_eq!(a.to(Side::Down, 1), Ok(ChunkPoint::new(0, 0, 0).unwrap()));
        assert_eq!(a.to(Side::Down, 2), Err(ChunkPoint::new(0, 31, 0).unwrap()));
    }
}
