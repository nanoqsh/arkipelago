use crate::chunk;
use serde::{Deserialize, Serialize};
use std::{fmt, ops};

#[derive(Copy, Clone, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(try_from = "u8", into = "u8")]
pub struct Height(u8);

impl Height {
    pub const HEIGHT: u8 = chunk::HEIGHT as u8;

    pub const fn new(height: u8) -> Option<Self> {
        match height {
            1..=Self::HEIGHT => Some(Self(height)),
            _ => None,
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for Height {
    type Error = u8;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        Self::new(val).ok_or(val)
    }
}

impl From<Height> for u8 {
    fn from(height: Height) -> Self {
        height.0
    }
}

impl Default for Height {
    fn default() -> Self {
        Self(1)
    }
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl ops::AddAssign<u8> for Height {
    fn add_assign(&mut self, rhs: u8) {
        *self = Self::new(self.0.saturating_add(rhs)).unwrap();
    }
}

impl ops::Add<u8> for Height {
    type Output = Self;

    fn add(mut self, rhs: u8) -> Self::Output {
        self += rhs;
        self
    }
}

impl ops::AddAssign for Height {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.0.saturating_add(rhs.0)).unwrap();
    }
}

impl ops::Add for Height {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
