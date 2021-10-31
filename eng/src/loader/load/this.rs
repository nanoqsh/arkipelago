use crate::loader::{format::Format, Error};

pub(crate) trait Load<'a> {
    const PATH: &'static str;
    type Format: Format;
    type Asset;

    fn load(self, raw: <Self::Format as Format>::Raw) -> Result<Self::Asset, Error>;
}
