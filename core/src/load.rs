use crate::{height::Height, path::Pass, prelude::Rotation, tile::TileList};
use serde::Deserialize;
use std::io;

const PATH: &str = "./assets/tiles.json";

#[derive(Debug)]
pub enum Error {
    PassesLen(usize),
    Io(io::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

#[derive(Deserialize)]
struct RawVariant {
    name: String,
    #[serde(default)]
    rotation: Rotation,
    passes: Vec<Pass>,
}

#[derive(Deserialize)]
struct RawTile<'a> {
    name: &'a str,
    height: Height,
    variants: Vec<RawVariant>,
}

fn load(tile: RawTile, list: &mut TileList) -> Result<(), Error> {
    let RawTile {
        name,
        height,
        variants,
    } = tile;

    for variant in &variants {
        let len = variant.passes.len();
        if height.get() as usize != len {
            return Err(Error::PassesLen(len));
        }
    }

    list.add(
        name,
        height,
        variants.into_iter().map(
            |RawVariant {
                 name,
                 rotation,
                 passes,
             }| (name, rotation, passes),
        ),
    );
    Ok(())
}

pub(crate) fn load_tiles(list: &mut TileList) -> Result<(), Error> {
    let content = std::fs::read_to_string(PATH)?;
    let tiles: Vec<RawTile> = serde_json::from_str(&content)?;
    tiles.into_iter().try_for_each(|tile| load(tile, list))
}
