use crate::{point::Error, side::Side};
use shr::cgm::*;
use std::fmt;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct ClusterPoint {
    x: i32,
    y: i32,
    z: i32,
}

impl ClusterPoint {
    pub const fn new(x: i32, y: i32, z: i32) -> Result<Self, Error> {
        if x == i32::MIN || y == i32::MIN || z == i32::MIN {
            return Err(Error);
        }

        Ok(Self { x, y, z })
    }

    pub const fn x(self) -> i32 {
        self.x
    }

    pub const fn y(self) -> i32 {
        self.y
    }

    pub const fn z(self) -> i32 {
        self.z
    }

    pub fn to(self, side: Side) -> Self {
        let (x, y, z) = self.into();
        match side {
            Side::Left => x
                .checked_add(1)
                .map(|x| Self { x, y, z })
                .unwrap_or_else(|| Self {
                    x: i32::MIN + 1,
                    y,
                    z,
                }),
            Side::Right => match x - 1 {
                i32::MIN => Self { x: i32::MAX, y, z },
                x => Self { x, y, z },
            },
            Side::Up => y
                .checked_add(1)
                .map(|y| Self { x, y, z })
                .unwrap_or_else(|| Self {
                    x,
                    y: i32::MIN + 1,
                    z,
                }),
            Side::Down => match y - 1 {
                i32::MIN => Self { x, y: i32::MAX, z },
                y => Self { x, y, z },
            },
            Side::Forth => z
                .checked_add(1)
                .map(|z| Self { x, y, z })
                .unwrap_or_else(|| Self {
                    x,
                    y,
                    z: i32::MIN + 1,
                }),
            Side::Back => match z - 1 {
                i32::MIN => Self { x, y, z: i32::MAX },
                z => Self { x, y, z },
            },
        }
    }
}

impl TryFrom<IVec3> for ClusterPoint {
    type Error = Error;

    fn try_from(vec: IVec3) -> Result<Self, Self::Error> {
        let (x, y, z) = vec.into();
        Self::new(x, y, z)
    }
}

impl From<ClusterPoint> for (i32, i32, i32) {
    fn from(ClusterPoint { x, y, z }: ClusterPoint) -> Self {
        (x, y, z)
    }
}

impl From<ClusterPoint> for IVec3 {
    fn from(point: ClusterPoint) -> Self {
        let (x, y, z) = point.into();
        Self::new(x, y, z)
    }
}

impl fmt::Display for ClusterPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self { x, y, z } = self;
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

impl fmt::Debug for ClusterPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
