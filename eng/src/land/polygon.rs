use serde::Deserialize;
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum Error {
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty"),
        }
    }
}

impl std::error::Error for Error {}

type Point = (f32, f32);

#[derive(Debug, Deserialize)]
#[serde(try_from = "Box<[Point]>")]
pub(crate) struct Polygon(Box<[Point]>);

impl Polygon {
    pub fn new<P>(points: P) -> Result<Self, Error>
    where
        P: Into<Box<[Point]>>,
    {
        let points = points.into();
        match points[..] {
            [] => Err(Error::Empty),
            _ => Ok(Self(points)),
        }
    }

    pub fn points(&self) -> &[Point] {
        &self.0
    }

    pub fn flipped(&self) -> Flipped {
        Flipped(self)
    }

    fn eq<R>(&self, mut rhs: R) -> bool
    where
        R: ExactSizeIterator<Item = Point>,
    {
        if self.points().len() != rhs.len() {
            return false;
        }

        let head = rhs.next().unwrap();
        let mut split = self.points().splitn(2, |point| *point == head);
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
        self.eq(rhs.points().iter().copied())
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
        self.eq(rhs
            .0
            .points()
            .iter()
            .rev()
            .copied()
            .map(|(x, y)| (1. - x, y)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq() {
        let a = Polygon::new([(0., 0.), (1., 1.)]);
        let b = Polygon::new([(0., 0.), (1., 1.)]);
        assert_eq!(a, b);
        assert_eq!(b, a);

        let a = Polygon::new([(0., 0.), (1., 1.)]);
        let b = Polygon::new([(1., 1.), (0., 0.)]);
        assert_eq!(a, b);
        assert_eq!(b, a);

        let a = Polygon::new([(0., 0.), (1., 1.), (2., 2.)]);
        let b = Polygon::new([(1., 1.), (2., 2.), (0., 0.)]);
        let c = Polygon::new([(2., 2.), (0., 0.), (1., 1.)]);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, c);
    }

    #[test]
    fn ne() {
        let a = Polygon::new([(0., 0.)]);
        let b = Polygon::new([(1., 1.)]);
        assert_ne!(a, b);

        let a = Polygon::new([(0., 0.)]);
        let b = Polygon::new([(0., 0.), (1., 1.)]);
        assert_ne!(a, b);
    }

    #[test]
    fn eq_flipped() {
        let a = Polygon::new([(0., 0.), (0., 1.), (0.5, 0.5)]).unwrap();
        let b = Polygon::new([(0.5, 0.5), (1., 1.), (1., 0.)]).unwrap();
        assert_eq!(a, b.flipped());
        assert_eq!(b, a.flipped());

        let a = Polygon::new([(0., 0.), (0., 1.), (1., 1.), (1., 0.)]).unwrap();
        let b = Polygon::new([(0., 0.), (0., 1.), (1., 1.), (1., 0.)]).unwrap();
        assert_eq!(a, b);
        assert_eq!(a, b.flipped());
    }
}
