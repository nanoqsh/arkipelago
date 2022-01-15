use crate::loader::ASSETS_PATH;
use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub(crate) struct Format<E> {
    buf: PathBuf,
    path: PathBuf,
    extension: PhantomData<E>,
}

impl<E> Format<E> {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            buf: PathBuf::new(),
            path: path.into(),
            extension: PhantomData,
        }
    }

    pub fn make_path(&mut self, name: &str) -> &Path
    where
        E: Extension,
    {
        self.buf.clear();
        self.buf.push(ASSETS_PATH);
        self.buf.push(&self.path);
        self.buf.push(name);
        self.buf.set_extension(E::EXT);
        println!("[ DEBUG ] Read: {:?}", self.buf);
        &self.buf
    }
}

pub(crate) trait Extension {
    const EXT: &'static str;
}

pub(crate) struct Json;

impl Extension for Json {
    const EXT: &'static str = "json";
}

pub(crate) struct Png;

impl Extension for Png {
    const EXT: &'static str = "png";
}
