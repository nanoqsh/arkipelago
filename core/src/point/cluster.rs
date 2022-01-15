use crate::{point::Error, side::Side};
use serde::{Deserialize, Serialize};
use shr::cgm::*;
use std::{fmt, ops};

#[derive(Copy, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(try_from = "(i32, i32, i32)", into = "(i32, i32, i32)")]
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

        Ok(unsafe { Self::new_unchecked(x, y, z) })
    }

    const unsafe fn new_unchecked(x: i32, y: i32, z: i32) -> Self {
        debug_assert!(x != i32::MIN);
        debug_assert!(y != i32::MIN);
        debug_assert!(z != i32::MIN);
        Self { x, y, z }
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

impl TryFrom<(i32, i32, i32)> for ClusterPoint {
    type Error = Error;

    fn try_from((x, y, z): (i32, i32, i32)) -> Result<Self, Self::Error> {
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
        write!(f, "[{x}, {y}, {z}]")
    }
}

impl fmt::Debug for ClusterPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl ops::AddAssign for ClusterPoint {
    fn add_assign(&mut self, rhs: Self) {
        let x = match self.x.wrapping_add(rhs.x) {
            i32::MIN => i32::MAX,
            x => x,
        };

        let y = match self.y.wrapping_add(rhs.y) {
            i32::MIN => i32::MAX,
            y => y,
        };

        let z = match self.z.wrapping_add(rhs.z) {
            i32::MIN => i32::MAX,
            z => z,
        };

        *self = unsafe { Self::new_unchecked(x, y, z) };
    }
}

impl ops::Add for ClusterPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::SubAssign for ClusterPoint {
    fn sub_assign(&mut self, rhs: Self) {
        let x = match self.x.wrapping_sub(rhs.x) {
            i32::MIN => i32::MAX,
            x => x,
        };

        let y = match self.y.wrapping_sub(rhs.y) {
            i32::MIN => i32::MAX,
            y => y,
        };

        let z = match self.z.wrapping_sub(rhs.z) {
            i32::MIN => i32::MAX,
            z => z,
        };

        *self = unsafe { Self::new_unchecked(x, y, z) };
    }
}

impl ops::Sub for ClusterPoint {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
