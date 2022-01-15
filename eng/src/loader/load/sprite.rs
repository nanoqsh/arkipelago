use crate::loader::{load::Load, read::ReadImage, Error};
use image::DynamicImage;

pub(crate) struct SpriteLoad {
    pub read: ReadImage,
}

impl Load for SpriteLoad {
    type Asset = DynamicImage;
    type Error = Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Error> {
        let image = self.read.read(name)?;
        Ok(image)
    }
}
