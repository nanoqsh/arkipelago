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

#[derive(Copy, Clone)]
pub(crate) enum Axis {
    X,
    Y,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(try_from = "(f32, f32)")]
pub(crate) struct Point(f32, f32);

impl Point {
    fn flipped_x(self) -> Self {
        Self(1. - self.0, self.1)
    }

    fn flipped_y(self) -> Self {
        Self(self.0, 1. - self.1)
    }

    fn rotated(self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Q0 => self,
            Rotation::Q1 => Self(1. - self.1, self.0),
            Rotation::Q2 => Self(1. - self.0, 1. - self.1),
            Rotation::Q3 => Self(self.1, 1. - self.0),
        }
    }
}

impl TryFrom<(f32, f32)> for Point {
    type Error = Error;

    fn try_from((x, y): (f32, f32)) -> Result<Self, Self::Error> {
        if x.is_nan() || y.is_nan() {
            Err(Error::Nan)
        } else {
            Ok(Self(x, y))
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "Box<[Point]>")]
pub(crate) struct Polygon {
    points: Box<[Point]>,
    symmetric: (bool, bool),
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
                    symmetric: (false, false),
                };

                polygon.symmetric = (
                    polygon == polygon.flipped_x(),
                    polygon == polygon.flipped_y(),
                );

                Ok(polygon)
            }
        }
    }

    pub fn flipped_x(&self) -> FlippedX {
        FlippedX(self)
    }

    pub fn flipped_y(&self) -> FlippedY {
        FlippedY(self)
    }

    pub fn rotated(&self, rotation: Rotation) -> Result<Self, &Self> {
        match rotation {
            Rotation::Q0 => return Err(self),
            Rotation::Q2 if self.symmetric.0 => return Err(self),
            _ => (),
        }

        let mut new = self.clone();
        for point in new.points.iter_mut() {
            *point = point.rotated(rotation);
        }

        new.symmetric = (new == new.flipped_x(), new == new.flipped_y());
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
pub(crate) struct FlippedX<'a>(&'a Polygon);

impl PartialEq<FlippedX<'_>> for Polygon {
    fn eq(&self, rhs: &FlippedX) -> bool {
        self.eq(rhs.0.points.iter().rev().copied().map(Point::flipped_x))
    }
}

#[derive(Debug)]
pub(crate) struct FlippedY<'a>(&'a Polygon);

impl PartialEq<FlippedY<'_>> for Polygon {
    fn eq(&self, rhs: &FlippedY) -> bool {
        self.eq(rhs.0.points.iter().rev().copied().map(Point::flipped_y))
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

    pub fn eq(&self, a: u16, b: u16, axis: Axis) -> bool {
        let lhs = self.get(a);
        match axis {
            Axis::X => {
                if lhs.symmetric.0 && a == b {
                    println!("[ DEBUG ] Skipped x");
                    return true;
                }

                let rhs = self.get(b);
                lhs == &rhs.flipped_x()
            }
            Axis::Y => {
                if lhs.symmetric.1 && a == b {
                    println!("[ DEBUG ] Skipped y");
                    return true;
                }

                let rhs = self.get(b);
                lhs == &rhs.flipped_y()
            }
        }
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
    fn eq_flipped_x() {
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
        assert_eq!(a, b.flipped_x());
        assert_eq!(b, a.flipped_x());

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
        assert_eq!(a, b.flipped_x());
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

        assert!(a.symmetric.0 && a.symmetric.1);
        assert!(b.symmetric.0 && b.symmetric.1);
        assert!(c.symmetric.0 && c.symmetric.1);
        assert!(d.symmetric.0 && d.symmetric.1);

        assert_eq!(&polygon, a);
        assert_eq!(&polygon, &b);
        assert_eq!(&polygon, c);
        assert_eq!(&polygon, &d);
    }
}
