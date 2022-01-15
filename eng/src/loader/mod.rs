mod format;
mod load;
mod read;
#[allow(dead_code)]
mod this;

pub(crate) use self::this::Loader;

pub(crate) type Error = Box<dyn std::error::Error>;

pub(crate) const ASSETS_PATH: &str = "./assets";
