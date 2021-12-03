use crate::loader::Error;
use image::DynamicImage;
use serde::Deserialize;
use std::{io::Read, marker::PhantomData, path::Path};

pub(crate) trait Format {
    const EXT: &'static str;
    type Raw;

    fn read(self, path: &Path) -> Result<Self::Raw, Error>;
}

pub(crate) struct Json<'a, T> {
    buf: &'a mut String,
    typ: PhantomData<T>,
}

impl<'a, T> Json<'a, T> {
    pub fn new(buf: &'a mut String) -> Self {
        Self {
            buf,
            typ: PhantomData,
        }
    }
}

impl<'a, T: Deserialize<'a>> Format for Json<'a, T> {
    const EXT: &'static str = "json";
    type Raw = T;

    fn read(self, path: &Path) -> Result<Self::Raw, Error> {
        let mut file = std::fs::File::open(path)?;
        self.buf.clear();
        file.read_to_string(self.buf)?;
        let raw = serde_json::from_str(self.buf)?;
        Ok(raw)
    }
}

pub(crate) struct Png;

impl Format for Png {
    const EXT: &'static str = "png";
    type Raw = DynamicImage;

    fn read(self, path: &Path) -> Result<Self::Raw, Error> {
        let file = std::fs::File::open(path)?;
        let dec = image::codecs::png::PngDecoder::new(file)?;
        let raw = DynamicImage::from_decoder(dec)?;
        Ok(raw)
    }
}
