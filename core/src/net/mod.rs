use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub name: String,
    pub pass: String,
}

#[derive(Debug)]
pub enum Error {
    Len,
    LimitReached,
    Bincode(bincode::Error),
}

impl From<bincode::Error> for Error {
    fn from(err: bincode::Error) -> Self {
        Self::Bincode(err)
    }
}

const LEN_BYTES: usize = std::mem::size_of::<u32>();
const MAX_LEN: u32 = 2048;

pub struct Packed(Vec<u8>);

impl Packed {
    pub fn new<T>(val: &T) -> Result<Self, Error>
    where
        T: Serialize,
    {
        let mut buf = Vec::with_capacity(32);
        buf.extend([0, 0, 0, 0]);
        bincode::serialize_into(&mut buf, val)?;
        let len = (buf.len() - LEN_BYTES) as u32;
        if len > MAX_LEN {
            return Err(Error::LimitReached);
        }

        buf[..LEN_BYTES].copy_from_slice(&len.to_be_bytes()[..]);
        Ok(Self(buf))
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

pub struct Unpacked(Vec<u8>);

impl Unpacked {
    pub fn new(len: u32) -> Result<Self, Error> {
        if len > MAX_LEN {
            return Err(Error::LimitReached);
        }

        Ok(Unpacked(vec![0; len as usize]))
    }

    pub fn bytes(&mut self) -> &mut [u8] {
        &mut self.0
    }

    pub fn to<T>(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        Ok(bincode::deserialize(&self.0)?)
    }
}
