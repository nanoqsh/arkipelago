use crate::{
    loader::{load::Load, read::ReadImage, Error},
    Render, Texture,
};

pub(crate) struct TextureLoad<'a> {
    pub read: ReadImage,
    pub ren: &'a Render,
}

impl Load for TextureLoad<'_> {
    type Asset = Texture;
    type Error = Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Error> {
        let image = self.read.read(name)?;
        Ok(self.ren.make_texture(&image))
    }
}
