use crate::{
    chunk::{HEIGHT, SIDE},
    point::*,
};
use std::fmt;

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
        let (mut clx, mut cly, mut clz) = (x / SIDE as i64, y / HEIGHT as i64, z / SIDE as i64);

        if x < 0 {
            clx -= 1;
        }

        if y < 0 {
            cly -= 1;
        }

        if z < 0 {
            clz -= 1;
        }

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
        write!(f, "[{}, {}, {}]", x, y, z)
    }
}

impl fmt::Debug for GlobalPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}
