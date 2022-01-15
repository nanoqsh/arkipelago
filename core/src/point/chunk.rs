use crate::{
    chunk::{HEIGHT, SIDE},
    point::Error,
    side::Side,
};
use serde::{Deserialize, Serialize};
use shr::cgm::*;
use std::fmt;

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "(u8, u8, u8)", into = "(u8, u8, u8)")]
pub struct ChunkPoint {
    x: u8,
    y: u8,
    z: u8,
}

impl ChunkPoint {
    pub const fn new(x: u8, y: u8, z: u8) -> Result<Self, Error> {
        if x < SIDE as u8 && y < HEIGHT as u8 && z < SIDE as u8 {
            Ok(unsafe { Self::new_unchecked(x, y, z) })
        } else {
            Err(Error)
        }
    }

    const unsafe fn new_unchecked(x: u8, y: u8, z: u8) -> Self {
        debug_assert!(x < SIDE as u8);
        debug_assert!(y < HEIGHT as u8);
        debug_assert!(z < SIDE as u8);
        Self { x, y, z }
    }

    pub const fn axes(self) -> (u8, u8, u8) {
        (self.x, self.y, self.z)
    }

    pub const fn x(self) -> u8 {
        self.x
    }

    pub const fn y(self) -> u8 {
        self.y
    }

    pub const fn z(self) -> u8 {
        self.z
    }

    /// Returns the point moved to `side` by `n`.
    /// If `Ok` returns, then the point is in this chunk,
    /// `Err` in the neighboring one.
    ///
    /// If `n >= SIDE` for Left, Right, Forth, Back side
    /// or `n >= HEIGHT` for Up, Down side then the function panics.
    pub fn to(self, side: Side, n: u8) -> Result<Self, Self> {
        let Self { x, y, z } = self;
        let res = match side {
            Side::Left => {
                assert!(n < SIDE as u8);

                let v = x + n;
                if v < SIDE as u8 {
                    Ok((v, y, z))
                } else {
                    Err((v - SIDE as u8, y, z))
                }
            }
            Side::Right => {
                assert!(n < SIDE as u8);

                if n <= x {
                    Ok((x - n, y, z))
                } else {
                    Err(((SIDE as u8) - n + x, y, z))
                }
            }
            Side::Up => {
                assert!(n < HEIGHT as u8);

                let v = y + n;
                if v < HEIGHT as u8 {
                    Ok((x, v, z))
                } else {
                    Err((x, v - HEIGHT as u8, z))
                }
            }
            Side::Down => {
                assert!(n < HEIGHT as u8);

                if n <= y {
                    Ok((x, y - n, z))
                } else {
                    Err((x, (HEIGHT as u8) - n + y, z))
                }
            }
            Side::Forth => {
                assert!(n < SIDE as u8);

                let v = z + n;
                if v < SIDE as u8 {
                    Ok((x, y, v))
                } else {
                    Err((x, y, v - SIDE as u8))
                }
            }
            Side::Back => {
                assert!(n < SIDE as u8);

                if n <= z {
                    Ok((x, y, z - n))
                } else {
                    Err((x, y, (SIDE as u8) - n + z))
                }
            }
        };

        unsafe {
            res.map(|(x, y, z)| Self::new_unchecked(x, y, z))
                .map_err(|(x, y, z)| Self::new_unchecked(x, y, z))
        }
    }

    pub fn wrapping_add(self, rhs: Self) -> Self {
        let (lx, ly, lz) = self.axes();
        let (rx, ry, rz) = rhs.axes();
        unsafe {
            Self::new_unchecked(
                (lx + rx) % SIDE as u8,
                (ly + ry) % HEIGHT as u8,
                (lz + rz) % SIDE as u8,
            )
        }
    }

    pub fn wrapping_sub(self, rhs: Self) -> Self {
        let (lx, ly, lz) = self.axes();
        let (rx, ry, rz) = rhs.axes();
        unsafe {
            Self::new_unchecked(
                if lx >= rx {
                    lx - rx
                } else {
                    (SIDE as u8) - rx + lx
                },
                if ly >= ry {
                    ly - ry
                } else {
                    (HEIGHT as u8) - ry + ly
                },
                if lz >= rz {
                    lz - rz
                } else {
                    (SIDE as u8) - rz + lz
                },
            )
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

impl TryFrom<(u8, u8, u8)> for ChunkPoint {
    type Error = Error;

    fn try_from((x, y, z): (u8, u8, u8)) -> Result<Self, Self::Error> {
        Self::new(x, y, z)
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

impl From<ChunkPoint> for Vec3 {
    fn from(point: ChunkPoint) -> Self {
        let (x, y, z) = point.axes();
        Self::new(x as f32, (y as f32) * 0.5, z as f32)
    }
}

impl fmt::Display for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.axes();
        write!(f, "[{x}, {y}, {z}]")
    }
}

impl fmt::Debug for ChunkPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

pub struct Points {
    x: u8,
    y: u8,
    z: u8,
    run: bool,
}

impl Points {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            run: true,
        }
    }
}

impl Iterator for Points {
    type Item = ChunkPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.run {
            return None;
        }

        let point = unsafe { ChunkPoint::new_unchecked(self.x, self.y, self.z) };

        self.y += 1;
        if self.y == HEIGHT as u8 {
            self.y = 0;
            self.z += 1;
            if self.z == SIDE as u8 {
                self.z = 0;
                self.x += 1;
                if self.x == SIDE as u8 {
                    self.run = false;
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
        assert_eq!(a.to(Side::Right, 0), Ok(a));
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
        assert_eq!(a.to(Side::Down, 0), Ok(a));
        assert_eq!(a.to(Side::Down, 1), Ok(ChunkPoint::new(0, 0, 0).unwrap()));
        assert_eq!(a.to(Side::Down, 2), Err(ChunkPoint::new(0, 31, 0).unwrap()));
    }
}
