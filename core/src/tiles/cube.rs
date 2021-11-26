use crate::{layout::Data, prelude::*};

pub struct Cube {
    variants: Vec<&'static str>,
}

impl Cube {
    pub fn new(variants: Vec<&'static str>) -> Self {
        Self { variants }
    }
}

impl Tile for Cube {
    fn height(&self) -> u8 {
        2
    }

    fn variants(&self) -> &[&'static str] {
        &self.variants
    }

    fn place(&self, _: &mut Cluster, point: GlobalPoint) -> Placement {
        println!("[ DEBUG ] Place a cube at {}", point);

        Placement {
            variant: VariantIndex(0),
            data: &[Data::None],
        }
    }
}
