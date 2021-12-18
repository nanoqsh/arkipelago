use crate::side::Side;
use serde::Deserialize;
use shr::cgm::Vec3;
use std::{error, fmt, ops};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Num(u8),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Num(num) => write!(f, "wrong number {}", num),
        }
    }
}

impl error::Error for ParseError {}

#[derive(Deserialize, Copy, Clone, Eq, Hash, PartialEq)]
#[serde(try_from = "u8")]
pub enum Rotation {
    Q0 = 0,
    Q1 = 1,
    Q2 = 2,
    Q3 = 3,
}

impl Rotation {
    pub const fn from_quarters(quarters: u8) -> Option<Self> {
        match quarters {
            0 => Some(Self::Q0),
            1 => Some(Self::Q1),
            2 => Some(Self::Q2),
            3 => Some(Self::Q3),
            _ => None,
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::Q0 => Self::Q2,
            Self::Q1 => Self::Q3,
            Self::Q2 => Self::Q0,
            Self::Q3 => Self::Q1,
        }
    }

    pub fn transform_vec(self, vec: Vec3) -> Vec3 {
        match self {
            Self::Q0 => vec,
            Self::Q1 => Vec3::new(vec.z, vec.y, -vec.x),
            Self::Q2 => Vec3::new(-vec.x, vec.y, -vec.z),
            Self::Q3 => Vec3::new(-vec.z, vec.y, vec.x),
        }
    }

    pub const fn rotate(self, side: Side) -> Side {
        match side {
            Side::Left => match self {
                Self::Q0 => Side::Left,
                Self::Q1 => Side::Back,
                Self::Q2 => Side::Right,
                Self::Q3 => Side::Forth,
            },
            Side::Right => match self {
                Self::Q0 => Side::Right,
                Self::Q1 => Side::Forth,
                Self::Q2 => Side::Left,
                Self::Q3 => Side::Back,
            },
            Side::Forth => match self {
                Self::Q0 => Side::Forth,
                Self::Q1 => Side::Left,
                Self::Q2 => Side::Back,
                Self::Q3 => Side::Right,
            },
            Side::Back => match self {
                Self::Q0 => Side::Back,
                Self::Q1 => Side::Right,
                Self::Q2 => Side::Forth,
                Self::Q3 => Side::Left,
            },
            _ => side,
        }
    }
}

impl From<Rotation> for Side {
    fn from(rotation: Rotation) -> Self {
        match rotation {
            Rotation::Q0 => Self::Forth,
            Rotation::Q1 => Self::Left,
            Rotation::Q2 => Self::Back,
            Rotation::Q3 => Self::Right,
        }
    }
}

impl TryFrom<u8> for Rotation {
    type Error = ParseError;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        Self::from_quarters(num).ok_or(ParseError::Num(num))
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Q0
    }
}

impl fmt::Display for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Q0 => "q0",
                Self::Q1 => "q1",
                Self::Q2 => "q2",
                Self::Q3 => "q3",
            }
        )
    }
}

impl fmt::Debug for Rotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ops::AddAssign for Rotation {
    fn add_assign(&mut self, rhs: Self) {
        let quarters = *self as u8 + rhs as u8;
        *self = Self::from_quarters(quarters % 4).unwrap()
    }
}

impl ops::Add for Rotation {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transform_vec_x() {
        let vec = Vec3::new(1., 0., 0.);
        assert_eq!(Rotation::Q0.transform_vec(vec), vec);
        assert_eq!(Rotation::Q1.transform_vec(vec), Vec3::new(0., 0., -1.));
        assert_eq!(Rotation::Q2.transform_vec(vec), Vec3::new(-1., 0., 0.));
        assert_eq!(Rotation::Q3.transform_vec(vec), Vec3::new(0., 0., 1.));
    }

    #[test]
    fn transform_vec_y() {
        let vec = Vec3::new(0., 1., 0.);
        assert_eq!(Rotation::Q0.transform_vec(vec), vec);
        assert_eq!(Rotation::Q1.transform_vec(vec), vec);
        assert_eq!(Rotation::Q2.transform_vec(vec), vec);
        assert_eq!(Rotation::Q3.transform_vec(vec), vec);
    }

    #[test]
    fn transform_vec_z() {
        let vec = Vec3::new(0., 0., 1.);
        assert_eq!(Rotation::Q0.transform_vec(vec), vec);
        assert_eq!(Rotation::Q1.transform_vec(vec), Vec3::new(1., 0., 0.));
        assert_eq!(Rotation::Q2.transform_vec(vec), Vec3::new(0., 0., -1.));
        assert_eq!(Rotation::Q3.transform_vec(vec), Vec3::new(-1., 0., 0.));
    }
}
