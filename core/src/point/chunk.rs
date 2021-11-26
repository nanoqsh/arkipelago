use crate::{
    chunk::{HEIGHT, SIDE},
    point::Error,
    prelude::Side,
};
use shr::cgm::*;
use std::fmt;

const CHUNK_SIDE_MAX: u8 = SIDE as u8 - 1;
const CHUNK_HEIGHT_MAX: u8 = HEIGHT as u8 - 1;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct ChunkPoint(u16);

impl ChunkPoint {
    pub fn new(x: u8, y: u8, z: u8) -> Result<Self, Error> {
        let (x, y, z) = match (x, y, z) {
            (0..=CHUNK_SIDE_MAX, 0..=CHUNK_HEIGHT_MAX, 0..=CHUNK_SIDE_MAX) => (x, y, z),
            _ => return Err(Error),
        };

        Ok(unsafe { Self::new_unchecked(x, y, z) })
    }

    unsafe fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x <= CHUNK_SIDE_MAX);
        debug_assert!(y <= CHUNK_HEIGHT_MAX);
        debug_assert!(z <= CHUNK_SIDE_MAX);
        Self(x as u16 | (y as u16) << 4 | (z as u16) << 9)
    }

    pub const fn axes(self) -> (u8, u8, u8) {
        (
            (self.0 & 0b1111) as u8,
            (self.0 >> 4 & 0b11111) as u8,
            (self.0 >> 9 & 0b1111) as u8,
        )
    }

    pub const fn x(self) -> u8 {
        self.axes().0
    }

    pub const fn y(self) -> u8 {
        self.axes().1
    }

    pub const fn z(self) -> u8 {
        self.axes().2
    }

    /// Returns the point moved to `side` by `n`.
    /// If `Ok` returns, then the point is in this chunk,
    /// `Err` in the neighboring one.
    pub fn to(self, side: Side, n: u8) -> Result<Self, Self> {
        let (x, y, z) = self.axes();
        match side {
            Side::Left => Self::new(x.saturating_add(n), y, z)
                .map_err(|_| Self::new(x.saturating_add(n) - SIDE as u8, y, z).unwrap()),
            Side::Right => {
                if x >= n {
                    Ok(Self::new(x - n, y, z).unwrap())
                } else {
                    Err(Self::new((SIDE as u8).wrapping_sub(n) + x, y, z).unwrap())
                }
            }
            Side::Up => Self::new(x, y.saturating_add(n), z)
                .map_err(|_| Self::new(x, y.saturating_add(n) - HEIGHT as u8, z).unwrap()),
            Side::Down => {
                if y >= n {
                    Ok(Self::new(x, y - n, z).unwrap())
                } else {
                    Err(Self::new(x, (HEIGHT as u8).wrapping_sub(n) + y, z).unwrap())
                }
            }
            Side::Forth => Self::new(x, y, z.saturating_add(n))
                .map_err(|_| Self::new(x, y, z.saturating_add(n) - SIDE as u8).unwrap()),
            Side::Back => {
                if z >= n {
                    Ok(Self::new(x, y, z - n).unwrap())
                } else {
                    Err(Self::new(x, y, (SIDE as u8).wrapping_sub(n) + z).unwrap())
                }
            }
        }
    }
}

impl TryFrom<UVec3> for ChunkPoint {
    type Error = Error;

    fn try_from(vec: UVec3) -> Result<Self, Self::Error> {
        let (x, y, z) = vec.into();
        if x >= SIDE as u32 || y >= HEIGHT as u32 || z >= SIDE as u32 {
            return Err(Error);
        }

        Self::new(x as u8, y as u8, z as u8)
    }
}

impl From<ChunkPoint> for (u8, u8, u8) {
    fn from(point: ChunkPoint) -> Self {
        point.axes()
    }
}

impl From<ChunkPoint> for UVec3 {
    fn from(point: ChunkPoint) -> Self {
        let (x, y, z) = point.axes();
        Self::new(x as u32, y as u32, z as u32)
    }
}

impl fmt::Display for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.axes();
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

impl fmt::Debug for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Points {
    x: u8,
    y: u8,
    z: u8,
}

impl Points {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Iterator for Points {
    type Item = ChunkPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let point = unsafe { ChunkPoint::new_unchecked(self.x, self.y, self.z) };

        self.y += 1;
        if self.y == HEIGHT as u8 {
            self.y = 0;
            self.z += 1;
            if self.z == SIDE as u8 {
                self.z = 0;
                self.x += 1;
                if self.x == SIDE as u8 {
                    self.x = 0;
                    return None;
                }
            }
        }

        Some(point)
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
