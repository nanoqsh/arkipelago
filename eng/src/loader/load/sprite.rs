use crate::loader::re::*;
use image::DynamicImage;

pub(crate) struct SpriteLoad;

impl Load<'_> for SpriteLoad {
    const PATH: &'static str = "textures";
    type Format = Png;
    type Asset = DynamicImage;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        match &raw {
            DynamicImage::ImageLuma8(_)
            | DynamicImage::ImageLumaA8(_)
            | DynamicImage::ImageRgb8(_)
            | DynamicImage::ImageRgba8(_) => (),
            _ => panic!("unsupported image format"),
        };

        Ok(raw)
    }
}
