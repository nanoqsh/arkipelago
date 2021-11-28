use core::prelude::Rotation;
use serde::Deserialize;
use std::{error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Error {
    Empty,
    Nan,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty"),
            Self::Nan => write!(f, "NaN value"),
        }
    }
}

impl error::Error for Error {}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(try_from = "(f32, f32)")]
pub(crate) struct Point(f32, f32);

impl Point {
    fn flipped(self) -> Self {
        Self(1. - self.0, self.1)
    }

    fn rotated(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Q0 => self,
            Rotation::Q1 => Self(self.1, self.0),
            Rotation::Q2 => self.flipped(),
            Rotation::Q3 => Self(self.1, self.0).flipped(),
        }
    }
}

impl TryFrom<(f32, f32)> for Point {
    type Error = Error;

    fn try_from((x, y): (f32, f32)) -> Result<Self, Self::Error> {
        if x.is_nan() || y.is_nan() {
            Err(Error::Nan)
        } else {
            Ok(Point(x, y))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "Box<[Point]>")]
pub(crate) struct Polygon {
    points: Box<[Point]>,
    y_symmetric: bool,
}

impl Polygon {
    pub fn new<P>(points: P) -> Result<Self, Error>
    where
        P: Into<Box<[Point]>>,
    {
        let points = points.into();
        match points[..] {
            [] => Err(Error::Empty),
            _ => {
                let mut polygon = Self {
                    points,
                    y_symmetric: false,
                };

                polygon.y_symmetric = polygon == polygon.flipped();
                Ok(polygon)
            }
        }
    }

    pub fn flipped(&self) -> Flipped {
        Flipped(self)
    }

    pub fn rotated(&self, rotation: Rotation) -> Result<Self, &Self> {
        match rotation {
            Rotation::Q0 => return Err(self),
            Rotation::Q2 if self.y_symmetric => return Err(self),
            _ => (),
        }

        let mut new = self.clone();
        for point in new.points.iter_mut() {
            *point = point.rotated(rotation);
        }

        if let Rotation::Q1 = rotation {
            new.points.reverse();
        }

        if let Rotation::Q1 | Rotation::Q3 = rotation {
            new.y_symmetric = new == new.flipped();
        }

        Ok(new)
    }

    fn eq<R>(&self, mut rhs: R) -> bool
    where
        R: ExactSizeIterator<Item = Point>,
    {
        if self.points.len() != rhs.len() {
            return false;
        }

        let head = rhs.next().unwrap();
        let mut split = self.points.splitn(2, |point| *point == head);
        let left = split.next().unwrap();
        let right = match split.next() {
            None => return false,
            Some(right) => right,
        };

        rhs.eq(right.iter().copied().chain(left.iter().copied()))
    }
}

impl PartialEq for Polygon {
    fn eq(&self, rhs: &Self) -> bool {
        self.eq(rhs.points.iter().copied())
    }
}

impl TryFrom<Box<[Point]>> for Polygon {
    type Error = Error;

    fn try_from(points: Box<[Point]>) -> Result<Self, Self::Error> {
        Self::new(points)
    }
}

#[derive(Debug)]
pub(crate) struct Flipped<'a>(&'a Polygon);

impl PartialEq<Flipped<'_>> for Polygon {
    fn eq(&self, rhs: &Flipped) -> bool {
        self.eq(rhs.0.points.iter().rev().copied().map(Point::flipped))
    }
}

#[derive(Default)]
pub(crate) struct Polygons(Vec<Polygon>);

impl Polygons {
    pub fn with_capacity(cap: usize) -> Self {
        Self(Vec::with_capacity(cap))
    }

    pub fn add(&mut self, polygon: Polygon) -> u16 {
        match self.0.iter().position(|p| p == &polygon) {
            Some(idx) => idx as u16,
            None => {
                let idx = self.0.len();
                assert!(idx < u16::MAX as usize - 1);
                self.0.push(polygon);
                idx as u16
            }
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit()
    }

    pub fn eq(&self, a: u16, b: u16) -> bool {
        let lhs = self.get(a);
        if lhs.y_symmetric && a == b {
            return true;
        }

        let rhs = self.get(b);
        lhs == &rhs.flipped()
    }

    pub fn get(&self, idx: u16) -> &Polygon {
        self.0.get(idx as usize).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let a = Polygon::new([(0., 0.).try_into().unwrap(), (1., 1.).try_into().unwrap()]);
        let b = Polygon::new([(0., 0.).try_into().unwrap(), (1., 1.).try_into().unwrap()]);
        assert_eq!(a, b);
        assert_eq!(b, a);

        let a = Polygon::new([(0., 0.).try_into().unwrap(), (1., 1.).try_into().unwrap()]);
        let b = Polygon::new([(1., 1.).try_into().unwrap(), (0., 0.).try_into().unwrap()]);
        assert_eq!(a, b);
        assert_eq!(b, a);

        let a = Polygon::new([
            (0., 0.).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
            (2., 2.).try_into().unwrap(),
        ]);
        let b = Polygon::new([
            (1., 1.).try_into().unwrap(),
            (2., 2.).try_into().unwrap(),
            (0., 0.).try_into().unwrap(),
        ]);
        let c = Polygon::new([
            (2., 2.).try_into().unwrap(),
            (0., 0.).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
        ]);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn ne() {
        let a = Polygon::new([(0., 0.).try_into().unwrap()]);
        let b = Polygon::new([(1., 1.).try_into().unwrap()]);
        assert_ne!(a, b);

        let a = Polygon::new([(0., 0.).try_into().unwrap()]);
        let b = Polygon::new([(0., 0.).try_into().unwrap(), (1., 1.).try_into().unwrap()]);
        assert_ne!(a, b);
    }

    #[test]
    fn eq_flipped() {
        let a = Polygon::new([
            (0., 0.).try_into().unwrap(),
            (0., 1.).try_into().unwrap(),
            (0.5, 0.5).try_into().unwrap(),
        ])
        .unwrap();
        let b = Polygon::new([
            (0.5, 0.5).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
            (1., 0.).try_into().unwrap(),
        ])
        .unwrap();
        assert_eq!(a, b.flipped());
        assert_eq!(b, a.flipped());

        let a = Polygon::new([
            (0., 0.).try_into().unwrap(),
            (0., 1.).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
            (1., 0.).try_into().unwrap(),
        ])
        .unwrap();
        let b = Polygon::new([
            (0., 0.).try_into().unwrap(),
            (0., 1.).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
            (1., 0.).try_into().unwrap(),
        ])
        .unwrap();
        assert_eq!(a, b);
        assert_eq!(a, b.flipped());
    }

    #[test]
    fn rotation() {
        let polygon = Polygon::new([
            (0., 0.).try_into().unwrap(),
            (0., 1.).try_into().unwrap(),
            (1., 1.).try_into().unwrap(),
            (1., 0.).try_into().unwrap(),
        ])
        .unwrap();

        let a = polygon.rotated(Rotation::Q0).unwrap_err();
        let b = polygon.rotated(Rotation::Q1).unwrap();
        let c = polygon.rotated(Rotation::Q2).unwrap_err();
        let d = polygon.rotated(Rotation::Q3).unwrap();

        assert!(a.y_symmetric);
        assert!(b.y_symmetric);
        assert!(c.y_symmetric);
        assert!(d.y_symmetric);

        assert_eq!(&polygon, a);
        assert_eq!(&polygon, &b);
        assert_eq!(&polygon, c);
        assert_eq!(&polygon, &d);
    }
}
