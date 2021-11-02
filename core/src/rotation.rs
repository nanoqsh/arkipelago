use crate::prelude::*;
use serde::Deserialize;
use shr::cgm::Vec3;
use std::fmt;

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

impl std::error::Error for ParseError {}

#[derive(Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(try_from = "u8")]
pub enum Rotation {
    Q0 = 0,
    Q1 = 1,
    Q2 = 2,
    Q3 = 3,
}

impl Rotation {
    pub const fn from_quarters(quarters: u8) -> Option<Self> {
        Some(match quarters {
            0 => Self::Q0,
            1 => Self::Q1,
            2 => Self::Q2,
            3 => Self::Q3,
            _ => return None,
        })
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
                Self::Q1 => Side::Forth,
                Self::Q2 => Side::Right,
                Self::Q3 => Side::Back,
            },
            Side::Right => match self {
                Self::Q0 => Side::Right,
                Self::Q1 => Side::Back,
                Self::Q2 => Side::Left,
                Self::Q3 => Side::Forth,
            },
            Side::Forth => match self {
                Self::Q0 => Side::Forth,
                Self::Q1 => Side::Right,
                Self::Q2 => Side::Back,
                Self::Q3 => Side::Left,
            },
            Side::Back => match self {
                Self::Q0 => Side::Back,
                Self::Q1 => Side::Left,
                Self::Q2 => Side::Forth,
                Self::Q3 => Side::Right,
            },
            _ => side,
        }
    }
}

impl TryFrom<u8> for Rotation {
    type Error = ParseError;

    fn try_from(num: u8) -> Result<Self, Self::Error> {
        Self::from_quarters(num).ok_or(ParseError::Num(num))
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
