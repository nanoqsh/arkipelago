use crate::{
    cluster::Cluster,
    layout::Data,
    tile::{Placement, Tile},
};
use core::{point::Point, prelude::VariantIndex};

pub struct Base {
    height: u8,
    variants: Vec<&'static str>,
}

impl Base {
    pub fn new(height: u8, variants: Vec<&'static str>) -> Self {
        Self { height, variants }
    }
}

impl Tile for Base {
    fn height(&self) -> u8 {
        self.height
    }

    fn variants(&self) -> &[&'static str] {
        &self.variants
    }

    fn place(&self, _: &mut Cluster, _: Point) -> Placement {
        Placement {
            variant: VariantIndex(0),
            data: &[
                Data::None,
                Data::None,
                Data::None,
                Data::None,
                Data::None,
                Data::None,
                Data::None,
                Data::None,
            ][0..self.height as usize - 1],
        }
    }
}
