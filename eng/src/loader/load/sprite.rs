use crate::loader::re::*;
use image::DynamicImage;

pub(crate) struct SpriteLoad;

impl Load<'_> for SpriteLoad {
    const PATH: &'static str = "textures";
    type Format = Png;
    type Asset = DynamicImage;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        Ok(raw)
    }
}
