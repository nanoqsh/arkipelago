mod cached;
mod format;
mod load;
mod this;

mod re {
    pub(crate) use crate::loader::{
        format::{Format, Json, Png},
        load::Load,
        Error, ASSETS_PATH,
    };

    pub use serde::Deserialize;
}

pub(crate) use self::this::Loader;

pub(crate) type Error = Box<dyn std::error::Error>;

pub(crate) const ASSETS_PATH: &str = "./assets";
