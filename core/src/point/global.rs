use crate::{
    chunk::{HEIGHT, SIDE},
    point::*,
    side::{Side, Sides},
};
use std::{error, fmt, num::ParseIntError, ops, str::FromStr};

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    X,
    Y,
    Z,
    TooManyAxes,
    Int(ParseIntError),
    Error(Error),
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::Int(err)
    }
}

impl From<Error> for ParseError {
    fn from(err: Error) -> Self {
        Self::Error(err)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::X => write!(f, "no x specified"),
            Self::Y => write!(f, "no y specified"),
            Self::Z => write!(f, "no z specified"),
            Self::TooManyAxes => write!(f, "too many axes"),
            Self::Int(int) => write!(f, "parse int {}", int),
            Self::Error(err) => write!(f, "point error {}", err),
        }
    }
}

impl error::Error for ParseError {}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct GlobalPoint {
    ch: ChunkPoint,
    cl: ClusterPoint,
}

impl GlobalPoint {
    pub const fn new(ch: ChunkPoint, cl: ClusterPoint) -> Self {
        Self { ch, cl }
    }

    pub fn from_absolute(x: i64, y: i64, z: i64) -> Result<Self, Error> {
        fn modl(i: i64, n: i64) -> i64 {
            (i % n + n) % n
        }

        let (chx, chy, chz) = (
            modl(x, SIDE as i64),
            modl(y, HEIGHT as i64),
            modl(z, SIDE as i64),
        );

        let (clx, cly, clz) = (
            if x < 0 { x + 1 - SIDE as i64 } else { x } / SIDE as i64,
            if y < 0 { y + 1 - HEIGHT as i64 } else { y } / HEIGHT as i64,
            if z < 0 { z + 1 - SIDE as i64 } else { z } / SIDE as i64,
        );

        Ok(Self::new(
            ChunkPoint::new(chx as u8, chy as u8, chz as u8).unwrap(),
            ClusterPoint::new(clx.try_into()?, cly.try_into()?, clz.try_into()?)?,
        ))
    }

    pub const fn chunk_point(self) -> ChunkPoint {
        self.ch
    }

    pub const fn cluster_point(self) -> ClusterPoint {
        self.cl
    }

    pub fn absolute_point(self) -> (i64, i64, i64) {
        let (chx, chy, chz) = self.ch.into();
        let (clx, cly, clz) = self.cl.into();

        (
            clx as i64 * SIDE as i64 + chx as i64,
            cly as i64 * HEIGHT as i64 + chy as i64,
            clz as i64 * SIDE as i64 + chz as i64,
        )
    }

    pub fn neighbors(self) -> Neighbors {
        Neighbors {
            center: self,
            sides: Sides::all(),
        }
    }
}

impl TryFrom<(i64, i64, i64)> for GlobalPoint {
    type Error = Error;

    fn try_from((x, y, z): (i64, i64, i64)) -> Result<Self, Self::Error> {
        Self::from_absolute(x, y, z)
    }
}

impl From<GlobalPoint> for (i64, i64, i64) {
    fn from(point: GlobalPoint) -> Self {
        point.absolute_point()
    }
}

impl fmt::Display for GlobalPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.absolute_point();
        write!(
            f,
            "{} {}{}{} {}",
            x,
            if y < 0 { "-" } else { "" },
            (y / 2).abs(),
            if y % 2 == 0 { "" } else { ".5" },
            z
        )
    }
}

impl fmt::Debug for GlobalPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (x, y, z) = self.absolute_point();
        write!(f, "{{{}, {}, {}}}", x, y, z)
    }
}

impl TryFrom<&str> for GlobalPoint {
    type Error = ParseError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        src.parse()
    }
}

impl FromStr for GlobalPoint {
    type Err = ParseError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let mut splited = src.split_whitespace();
        let x = splited.next().ok_or(ParseError::X)?;
        let y = splited.next().ok_or(ParseError::Y)?;
        let z = splited.next().ok_or(ParseError::Z)?;
        if splited.next().is_some() {
            return Err(ParseError::TooManyAxes);
        }

        let x = x.parse()?;
        let (n, mut v): (i64, i64) = if let Some(stripped) = y.strip_suffix(".5") {
            (stripped.parse()?, 1)
        } else if let Some(stripped) = y.strip_suffix(".0") {
            (stripped.parse()?, 0)
        } else {
            (y.parse()?, 0)
        };
        let z = z.parse()?;

        if y.starts_with('-') {
            v = -v;
        }

        Ok(Self::from_absolute(x, n * 2 + v, z)?)
    }
}

impl ops::AddAssign<ChunkPoint> for GlobalPoint {
    fn add_assign(&mut self, rhs: ChunkPoint) {
        let ch = self.ch;
        let mut cl = self.cl;
        if ch.x() + rhs.x() >= SIDE as u8 {
            cl = cl.to(Side::Left);
        }

        if ch.y() + rhs.y() >= HEIGHT as u8 {
            cl = cl.to(Side::Up);
        }

        if ch.z() + rhs.z() >= SIDE as u8 {
            cl = cl.to(Side::Forth);
        }

        *self = Self::new(ch.wrapping_add(rhs), cl);
    }
}

impl ops::Add<ChunkPoint> for GlobalPoint {
    type Output = Self;

    fn add(mut self, rhs: ChunkPoint) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::SubAssign<ChunkPoint> for GlobalPoint {
    fn sub_assign(&mut self, rhs: ChunkPoint) {
        let ch = self.ch;
        let mut cl = self.cl;
        if ch.x() < rhs.x() {
            cl = cl.to(Side::Right);
        }

        if ch.y() < rhs.y() {
            cl = cl.to(Side::Down);
        }

        if ch.z() < rhs.z() {
            cl = cl.to(Side::Back);
        }

        *self = Self::new(ch.wrapping_sub(rhs), cl);
    }
}

impl ops::Sub<ChunkPoint> for GlobalPoint {
    type Output = Self;

    fn sub(mut self, rhs: ChunkPoint) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::AddAssign<ClusterPoint> for GlobalPoint {
    fn add_assign(&mut self, rhs: ClusterPoint) {
        *self = Self::new(self.ch, self.cl + rhs);
    }
}

impl ops::Add<ClusterPoint> for GlobalPoint {
    type Output = Self;

    fn add(mut self, rhs: ClusterPoint) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::SubAssign<ClusterPoint> for GlobalPoint {
    fn sub_assign(&mut self, rhs: ClusterPoint) {
        *self = Self::new(self.ch, self.cl - rhs);
    }
}

impl ops::Sub<ClusterPoint> for GlobalPoint {
    type Output = Self;

    fn sub(mut self, rhs: ClusterPoint) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::AddAssign for GlobalPoint {
    fn add_assign(&mut self, rhs: Self) {
        *self += rhs.ch;
        *self += rhs.cl;
    }
}

impl ops::Add for GlobalPoint {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::SubAssign for GlobalPoint {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= rhs.ch;
        *self -= rhs.cl;
    }
}

impl ops::Sub for GlobalPoint {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

pub struct Neighbors {
    center: GlobalPoint,
    sides: Sides,
}

impl Iterator for Neighbors {
    type Item = GlobalPoint;

    fn next(&mut self) -> Option<Self::Item> {
        let side = self.sides.next()?;
        match self.center.ch.to(side, 1) {
            Ok(ch) => Some(GlobalPoint::new(ch, self.center.cl)),
            Err(ch) => Some(GlobalPoint::new(ch, self.center.cl.to(side))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute() {
        let gl = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        assert_eq!(gl.absolute_point(), (0, 0, 0));

        let gl = GlobalPoint::from_absolute(1, -1, 0).unwrap();
        assert_eq!(gl.absolute_point(), (1, -1, 0));

        let gl = GlobalPoint::from_absolute(16, 0, -16).unwrap();
        assert_eq!(gl.absolute_point(), (16, 0, -16));

        let gl = GlobalPoint::from_absolute(-45, 50, 32).unwrap();
        assert_eq!(gl.absolute_point(), (-45, 50, 32));
    }

    #[test]
    fn to_string() {
        let point = GlobalPoint::from_absolute(1, -2, -1).unwrap();
        assert_eq!(point.to_string(), "1 -1 -1");

        let point = GlobalPoint::from_absolute(1, -1, -1).unwrap();
        assert_eq!(point.to_string(), "1 -0.5 -1");

        let point = GlobalPoint::from_absolute(1, 0, -1).unwrap();
        assert_eq!(point.to_string(), "1 0 -1");

        let point = GlobalPoint::from_absolute(1, 1, -1).unwrap();
        assert_eq!(point.to_string(), "1 0.5 -1");

        let point = GlobalPoint::from_absolute(1, 2, -1).unwrap();
        assert_eq!(point.to_string(), "1 1 -1");
    }

    #[test]
    fn parse() {
        let point: GlobalPoint = "1 -1 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, -2, -1).unwrap());

        let point: GlobalPoint = "1 -1.0 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, -2, -1).unwrap());

        let point: GlobalPoint = "1 -0.5 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, -1, -1).unwrap());

        let point: GlobalPoint = "1 -0 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, 0, -1).unwrap());

        let point: GlobalPoint = "1 0 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, 0, -1).unwrap());

        let point: GlobalPoint = "1 0.5 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, 1, -1).unwrap());

        let point: GlobalPoint = "1 1 -1".parse().unwrap();
        assert_eq!(point, GlobalPoint::from_absolute(1, 2, -1).unwrap());
    }

    #[test]
    fn parse_error() {
        let err = "".parse::<GlobalPoint>().unwrap_err();
        assert_eq!(err, ParseError::X);

        let err = "1".parse::<GlobalPoint>().unwrap_err();
        assert_eq!(err, ParseError::Y);

        let err = "1 1".parse::<GlobalPoint>().unwrap_err();
        assert_eq!(err, ParseError::Z);

        let err = "1 1 1 1".parse::<GlobalPoint>().unwrap_err();
        assert_eq!(err, ParseError::TooManyAxes);
    }

    #[test]
    fn add_chunk_point() {
        let gl = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        let ch = ChunkPoint::new(1, 1, 1).unwrap();
        assert_eq!(gl + ch, GlobalPoint::from_absolute(1, 1, 1).unwrap());

        let gl = GlobalPoint::from_absolute(1, 1, 1).unwrap();
        let ch = ChunkPoint::new(15, 31, 15).unwrap();
        assert_eq!(gl + ch, GlobalPoint::from_absolute(16, 32, 16).unwrap());
    }

    #[test]
    fn sub_chunk_point() {
        let gl = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        let ch = ChunkPoint::new(1, 1, 1).unwrap();
        assert_eq!(gl - ch, GlobalPoint::from_absolute(-1, -1, -1).unwrap());
    }

    #[test]
    fn add_cluster_point() {
        let gl = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        let cl = ClusterPoint::new(1, 1, 1).unwrap();
        assert_eq!(gl + cl, GlobalPoint::from_absolute(16, 32, 16).unwrap());
    }

    #[test]
    fn sub_cluster_point() {
        let gl = GlobalPoint::from_absolute(0, 0, 0).unwrap();
        let cl = ClusterPoint::new(1, 1, 1).unwrap();
        assert_eq!(gl - cl, GlobalPoint::from_absolute(-16, -32, -16).unwrap());
    }

    #[test]
    fn add() {
        let a = GlobalPoint::from_absolute(1, -1, 0).unwrap();
        let b = GlobalPoint::from_absolute(1, 1, 0).unwrap();
        assert_eq!(a + b, GlobalPoint::from_absolute(2, 0, 0).unwrap());
    }

    #[test]
    fn sub() {
        let a = GlobalPoint::from_absolute(1, -1, 0).unwrap();
        let b = GlobalPoint::from_absolute(1, 1, 0).unwrap();
        assert_eq!(a - b, GlobalPoint::from_absolute(0, -2, 0).unwrap());
    }

    #[test]
    fn neighbors() {
        use std::collections::HashSet;

        let neighbors: HashSet<_> = GlobalPoint::from_absolute(0, 0, 0)
            .unwrap()
            .neighbors()
            .into_iter()
            .collect();

        assert!(neighbors.contains(&GlobalPoint::from_absolute(1, 0, 0).unwrap()));
        assert!(neighbors.contains(&GlobalPoint::from_absolute(-1, 0, 0).unwrap()));
        assert!(neighbors.contains(&GlobalPoint::from_absolute(0, 1, 0).unwrap()));
        assert!(neighbors.contains(&GlobalPoint::from_absolute(0, -1, 0).unwrap()));
        assert!(neighbors.contains(&GlobalPoint::from_absolute(0, 0, 1).unwrap()));
        assert!(neighbors.contains(&GlobalPoint::from_absolute(0, 0, -1).unwrap()));
    }
}
