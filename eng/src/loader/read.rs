use crate::loader::{
    format::{Format, Json, Png},
    Error,
};
use image::DynamicImage;
use std::{fs, io::Read};

pub(crate) struct ReadJson {
    format: Format<Json>,
    content: String,
}

impl ReadJson {
    pub fn new(path: &str) -> Self {
        Self {
            format: Format::new(path),
            content: String::new(),
        }
    }

    pub fn read(&mut self, name: &str) -> Result<&str, Error> {
        let path = self.format.make_path(name);
        let mut file = fs::File::open(path)?;
        self.content.clear();
        file.read_to_string(&mut self.content)?;
        Ok(&self.content)
    }
}

pub(crate) struct ReadImage {
    format: Format<Png>,
}

impl ReadImage {
    pub fn new(path: &str) -> Self {
        Self {
            format: Format::new(path),
        }
    }

    pub fn read(&mut self, name: &str) -> Result<DynamicImage, image::ImageError> {
        let path = self.format.make_path(name);
        let file = std::fs::File::open(path)?;
        let decoder = image::codecs::png::PngDecoder::new(file)?;
        DynamicImage::from_decoder(decoder)
    }
}
