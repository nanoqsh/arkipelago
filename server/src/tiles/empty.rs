use crate::{
    cluster::Cluster,
    tile::{Placement, Tile},
};
use core::point::Point;

pub struct Empty;

impl Tile for Empty {
    fn height(&self) -> u8 {
        unreachable!()
    }

    fn variants(&self) -> &[&'static str] {
        unreachable!()
    }

    fn place(&self, _: &mut Cluster, _: Point) -> Placement {
        unreachable!()
    }
}
