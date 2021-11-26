use crate::prelude::*;

pub struct Empty;

impl Tile for Empty {
    fn height(&self) -> u8 {
        unreachable!()
    }

    fn variants(&self) -> &[&'static str] {
        unreachable!()
    }

    fn place(&self, _: &mut Cluster, _: GlobalPoint) -> Placement {
        unreachable!()
    }
}