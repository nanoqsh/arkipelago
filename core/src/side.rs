use serde::Deserialize;
use shr::cgm::*;
use std::{error, fmt, ops, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    Char(char),
    EmptyString,
    TooManySides,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Char(ch) => write!(f, "wrong char {}", ch),
            Self::EmptyString => write!(f, "empty string"),
            Self::TooManySides => write!(f, "too many sides"),
        }
    }
}

impl error::Error for ParseError {}

#[derive(Debug, Deserialize, Copy, Clone, Eq, Hash, PartialEq)]
#[serde(try_from = "&str")]
pub enum Side {
    Left = 0,
    Right = 1,
    Up = 2,
    Down = 3,
    Forth = 4,
    Back = 5,
}

impl Side {
    pub const ENUM: [Self; 6] = [
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::Forth,
        Self::Back,
    ];

    pub const AXES: [(Self, Self); 3] = [
        (Self::Left, Self::Right),
        (Self::Up, Self::Down),
        (Self::Forth, Self::Back),
    ];

    /// Returns directions of the vector.
    pub fn directions(vec: Vec3) -> [Option<Self>; 3] {
        use std::cmp::Ordering;

        vec.zip(Self::AXES.into(), |val, (hi, lo)| {
            match val.partial_cmp(&0.).unwrap() {
                Ordering::Less => Some(lo),
                Ordering::Equal => None,
                Ordering::Greater => Some(hi),
            }
        })
        .into()
    }

    /// Returns the nearest `Side` from the vector.
    pub fn nearest(vec: Vec3) -> Option<Self> {
        let arr: [f32; 3] = vec.map(f32::abs).into();
        Self::directions(vec)
            .into_iter()
            .zip(arr)
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(side, _)| side)
            .unwrap()
    }

    pub const fn to_vec(self) -> Vec3 {
        match self {
            Self::Left => Vec3::new(1., 0., 0.),
            Self::Right => Vec3::new(-1., 0., 0.),
            Self::Up => Vec3::new(0., 1., 0.),
            Self::Down => Vec3::new(0., -1., 0.),
            Self::Forth => Vec3::new(0., 0., 1.),
            Self::Back => Vec3::new(0., 0., -1.),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Forth => Self::Back,
            Self::Back => Self::Forth,
        }
    }
}

impl From<Side> for Vec3 {
    fn from(side: Side) -> Self {
        side.to_vec()
    }
}

impl TryFrom<char> for Side {
    type Error = ParseError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        Ok(match ch {
            'l' => Self::Left,
            'r' => Self::Right,
            'u' => Self::Up,
            'd' => Self::Down,
            'f' => Self::Forth,
            'b' => Self::Back,
            _ => return Err(ParseError::Char(ch)),
        })
    }
}

impl TryFrom<&str> for Side {
    type Error = ParseError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        src.parse()
    }
}

impl FromStr for Side {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut chars = src.chars();
        let side = chars
            .next()
            .map(TryFrom::try_from)
            .ok_or(ParseError::EmptyString)?;

        match chars.next() {
            None => side,
            Some(_) => Err(ParseError::TooManySides),
        }
    }
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Left => 'l',
                Self::Right => 'r',
                Self::Up => 'u',
                Self::Down => 'd',
                Self::Forth => 'f',
                Self::Back => 'b',
            }
        )
    }
}

impl<S: Into<Sides>> ops::BitOr<S> for Side {
    type Output = Sides;

    fn bitor(self, rhs: S) -> Self::Output {
        let lhs: Sides = self.into();
        lhs | rhs.into()
    }
}

#[derive(Default, Deserialize, Copy, Clone, Eq, Hash, PartialEq)]
#[serde(try_from = "&str")]
pub struct Sides(u8);

impl Sides {
    const ALL: Self = Self(0b0011_1111);

    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn all() -> Self {
        Self::ALL
    }

    pub const fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn contains(self, side: Side) -> bool {
        let sides: Self = side.into();
        self.0 & sides.0 != 0
    }

    pub fn remove(&mut self, side: Side) {
        self.0 &= !(1 << side as u8);
    }
}

impl From<Side> for Sides {
    fn from(side: Side) -> Self {
        Self(1 << side as u8)
    }
}

impl TryFrom<&str> for Sides {
    type Error = ParseError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        src.parse()
    }
}

impl FromStr for Sides {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        match src {
            "." => Ok(Self::all()),
            _ => src.chars().map(TryInto::try_into).collect(),
        }
    }
}

impl fmt::Display for Sides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for side in *self {
            write!(f, "{}", side)?;
        }
        write!(f, "]")
    }
}

impl fmt::Debug for Sides {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ops::Not for Sides {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.0 ^= Self::ALL.0;
        self
    }
}

impl<S: Into<Self>> ops::BitOrAssign<S> for Sides {
    fn bitor_assign(&mut self, rhs: S) {
        let rhs = rhs.into();
        self.0 |= rhs.0
    }
}

impl<S: Into<Self>> ops::BitOr<S> for Sides {
    type Output = Self;

    fn bitor(mut self, rhs: S) -> Self::Output {
        self |= rhs.into();
        self
    }
}

impl ops::BitAndAssign for Sides {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl ops::BitAnd for Sides {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl FromIterator<Side> for Sides {
    fn from_iter<T: IntoIterator<Item = Side>>(iter: T) -> Self {
        iter.into_iter().fold(Self::empty(), std::ops::BitOr::bitor)
    }
}

impl Iterator for Sides {
    type Item = Side;

    fn next(&mut self) -> Option<Self::Item> {
        for side in Side::ENUM {
            if self.contains(side) {
                self.remove(side);
                return Some(side);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_x() {
        let vec = Vec3::new(1., 0., 0.);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Left);

        let vec = Vec3::new(-1., 0., 0.);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Right);
    }

    #[test]
    fn nearest_y() {
        let vec = Vec3::new(1., 1.2, 1.);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Up);

        let vec = Vec3::new(1., -1.2, 1.);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Down);
    }

    #[test]
    fn nearest_z() {
        let vec = Vec3::new(0.2, 0.2, 1.5);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Forth);

        let vec = Vec3::new(0.2, 0.2, -1.5);
        let side = Side::nearest(vec).unwrap();
        assert_eq!(side, Side::Back);
    }

    #[test]
    fn side_from_str() {
        let side: Side = "u".parse().unwrap();
        assert_eq!(side, Side::Up);

        let err = "".parse::<Side>().unwrap_err();
        assert_eq!(err, ParseError::EmptyString);

        let err = "q".parse::<Side>().unwrap_err();
        assert_eq!(err, ParseError::Char('q'));

        let err = "lr".parse::<Side>().unwrap_err();
        assert_eq!(err, ParseError::TooManySides);
    }

    #[test]
    fn sides_from_str() {
        let sides: Sides = "u".parse().unwrap();
        assert_eq!(sides, Sides::empty() | Side::Up);

        let sides: Sides = "uuu".parse().unwrap();
        assert_eq!(sides, Sides::empty() | Side::Up);

        let sides: Sides = "lrudfb".parse().unwrap();
        assert_eq!(sides, Sides::all());

        let sides: Sides = ".".parse().unwrap();
        assert_eq!(sides, Sides::all());
    }
}
