use crate::{loader::re::*, Render, Texture};
use image::{DynamicImage, GenericImageView};

pub(crate) struct TextureLoad<'a> {
    pub ren: &'a Render,
}

impl<'a> Load<'a> for TextureLoad<'a> {
    const PATH: &'static str = "textures";
    type Format = Png;
    type Asset = Texture;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error> {
        use ngl::texture::*;

        let (data, format) = match &raw {
            DynamicImage::ImageLuma8(data) => (data.as_raw(), Format::R),
            DynamicImage::ImageLumaA8(data) => (data.as_raw(), Format::Rg),
            DynamicImage::ImageRgb8(data) => (data.as_raw(), Format::Rgb),
            DynamicImage::ImageRgba8(data) => (data.as_raw(), Format::Rgba),
            _ => panic!("unsupported image format"),
        };

        let size = raw.dimensions();
        let params = Parameters {
            format,
            ..Parameters::default()
        };

        Ok(self.ren.make_texture(data, size.into(), params))
    }
}
