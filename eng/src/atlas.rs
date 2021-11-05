use image::{DynamicImage, GenericImage, GenericImageView};
use shr::cgm::*;

#[derive(Debug)]
pub enum Error {
    Dimensions(u32, u32),
    NoSprites,
}

pub struct Mapper {
    size: u32,
    multiplier: f32,
}

impl Mapper {
    fn new(size: u32) -> Self {
        Self {
            size,
            multiplier: 1. / size as f32,
        }
    }

    pub fn addition(&self, sprite: u32) -> Vec2 {
        let x = sprite % self.size;
        let y = sprite / self.size;
        Vec2::new(x as f32, y as f32)
    }

    pub fn multiplier(&self) -> f32 {
        self.multiplier
    }
}

pub struct Atlas {
    map: DynamicImage,
    size: u32,
}

impl Atlas {
    pub fn new<'a, S>(sprites: S) -> Result<Self, Error>
    where
        S: IntoIterator<Item = &'a DynamicImage>,
        S::IntoIter: ExactSizeIterator,
    {
        let mut sprites = sprites.into_iter().peekable();
        let sprite = sprites.peek().ok_or(Error::NoSprites)?;
        let (width, height) = sprite.dimensions();
        if width != height {
            return Err(Error::Dimensions(width, height));
        }

        let size = (sprites.len() as f32).sqrt().ceil() as u32;
        let sprite_size = width;
        let mut map = {
            let pixel_size = size * sprite_size;
            DynamicImage::new_rgba8(pixel_size, pixel_size)
        };

        for (i, sprite) in sprites.enumerate() {
            let (width, height) = sprite.dimensions();
            if width != sprite_size || height != sprite_size {
                return Err(Error::Dimensions(width, height));
            }

            let i = i as u32;
            let x = (i % size) * sprite_size;
            let y = (i / size) * sprite_size;
            map.copy_from(sprite, x, y).unwrap();
        }

        Ok(Self { map, size })
    }

    pub fn map(self) -> (DynamicImage, Mapper) {
        (self.map, Mapper::new(self.size))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgba};

    const SIZE: u32 = 4;
    const RED: [u8; 4] = [255, 0, 0, 255];
    const GREEN: [u8; 4] = [0, 255, 0, 255];
    const BLUE: [u8; 4] = [0, 0, 255, 255];
    const WHITE: [u8; 4] = [255, 255, 255, 255];

    fn sprite_from_size(col: [u8; 4], (width, height): (u32, u32)) -> DynamicImage {
        DynamicImage::ImageRgba8(ImageBuffer::from_pixel(width, height, Rgba(col)))
    }

    fn sprite(col: [u8; 4]) -> DynamicImage {
        sprite_from_size(col, (SIZE, SIZE))
    }

    #[test]
    fn atlas_2x2() {
        let sprites = [sprite(RED), sprite(GREEN), sprite(BLUE), sprite(WHITE)];
        let atlas = Atlas::new(&sprites).unwrap();

        let (map, _) = atlas.map();
        assert_eq!(map.get_pixel(0, 0), Rgba(RED));
        assert_eq!(map.get_pixel(SIZE - 1, SIZE - 1), Rgba(RED));
        assert_eq!(map.get_pixel(SIZE, 0), Rgba(GREEN));
        assert_eq!(map.get_pixel(0, SIZE), Rgba(BLUE));
        assert_eq!(map.get_pixel(SIZE, SIZE), Rgba(WHITE));
        assert_eq!(map.width(), SIZE * 2);
        assert_eq!(map.height(), SIZE * 2);
    }

    #[test]
    fn atlas_3x3() {
        let sprites = [
            sprite(RED),
            sprite(GREEN),
            sprite(BLUE),
            sprite(WHITE),
            sprite(RED),
        ];
        let atlas = Atlas::new(&sprites).unwrap();

        let (map, _) = atlas.map();
        assert_eq!(map.get_pixel(0, 0), Rgba(RED));
        assert_eq!(map.get_pixel(SIZE - 1, SIZE - 1), Rgba(RED));
        assert_eq!(map.get_pixel(SIZE, 0), Rgba(GREEN));
        assert_eq!(map.get_pixel(SIZE * 2, 0), Rgba(BLUE));
        assert_eq!(map.get_pixel(0, SIZE), Rgba(WHITE));
        assert_eq!(map.get_pixel(SIZE, SIZE), Rgba(RED));
        assert_eq!(map.width(), SIZE * 3);
        assert_eq!(map.height(), SIZE * 3);
    }

    #[test]
    fn map() {
        let sprites = [sprite(RED), sprite(GREEN), sprite(BLUE), sprite(WHITE)];
        let atlas = Atlas::new(&sprites).unwrap();
        let (_, mapper) = atlas.map();
        let src = Vec2::new(1., 1.);

        let res = (src + mapper.addition(0)) * mapper.multiplier();
        assert_eq!(res, Vec2::new(0.5, 0.5));
        let res = (src + mapper.addition(1)) * mapper.multiplier();
        assert_eq!(res, Vec2::new(1., 0.5));
        let res = (src + mapper.addition(2)) * mapper.multiplier();
        assert_eq!(res, Vec2::new(0.5, 1.));
        let res = (src + mapper.addition(3)) * mapper.multiplier();
        assert_eq!(res, Vec2::new(1., 1.));
    }

    #[test]
    fn wrong_dimensions() {
        let sprites = [
            sprite_from_size(RED, (8, 8)),
            sprite_from_size(GREEN, (16, 16)),
        ];
        let err = Atlas::new(&sprites);
        assert!(matches!(err, Err(Error::Dimensions(16, 16))));
    }

    #[test]
    fn no_sprites() {
        let err = Atlas::new(&[]);
        assert!(matches!(err, Err(Error::NoSprites)));
    }
}
