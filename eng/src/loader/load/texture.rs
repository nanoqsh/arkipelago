use crate::{loader::re::*, Render, Texture};

pub(crate) struct TextureLoad<'a> {
    pub ren: &'a Render,
}

impl Load<'_> for TextureLoad<'_> {
    const PATH: &'static str = "textures";
    type Format = Png;
    type Asset = Texture;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        Ok(self.ren.make_texture(&raw))
    }
}
