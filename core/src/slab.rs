/// Slab layout.
///
/// Can be a empty, base or truck.
/// Empty -> all bits are 0.
/// Base -> tttttttt_tttttttt_0lll0hhh_vvvvvvvv
/// where
///     t: tile
///     l: level (always 0 for base)
///     h: height (actual height - 1)
///     v: variant
/// Trunk -> tttttttt_tttttttt_0llldddd_dddddddd
/// where
///     t: tile
///     l: level
///     d: data
#[derive(Copy, Clone)]
pub(crate) struct Slab(u16, u16);

impl Slab {
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn typed(self) -> Typed {
        match self {
            Self(0, _) => Typed::Empty(Empty),
            Self(a, b) if b >> 12 & 0b111 == 0 => Typed::Base(Base(a, b)),
            Self(a, b) => Typed::Trunk(Trunk(a, b)),
        }
    }
}

impl From<Empty> for Slab {
    fn from(_: Empty) -> Self {
        Self(0, 0)
    }
}

impl From<Base> for Slab {
    fn from(Base(a, b): Base) -> Self {
        Self(a, b)
    }
}

impl From<Trunk> for Slab {
    fn from(Trunk(a, b): Trunk) -> Self {
        Self(a, b)
    }
}

pub(crate) enum Typed {
    Empty(Empty),
    Base(Base),
    Trunk(Trunk),
}

#[derive(Copy, Clone)]
pub(crate) struct Empty;

impl Empty {
    pub const fn new() -> Self {
        Self
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Base(u16, u16);

impl Base {
    pub const fn new(tile: u16, variant: u8, height: u8) -> Self {
        Self(tile, variant as u16 | ((height - 1) as u16) << 8)
    }

    pub const fn tile(self) -> u16 {
        self.0
    }

    pub const fn variant(self) -> u8 {
        self.1 as u8
    }

    pub const fn level(self) -> u8 {
        (self.1 >> 12) as u8 & 0b111
    }

    pub const fn height(self) -> u8 {
        (self.1 >> 8) as u8 & 0b111
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Trunk(u16, u16);

impl Trunk {
    pub const fn new(tile: u16, data: u16, level: u8) -> Self {
        Self(tile, data | (level as u16) << 12)
    }

    pub const fn tile(self) -> u16 {
        self.0
    }

    pub const fn data(self) -> u16 {
        self.1 & 0b111111111111
    }

    pub const fn level(self) -> u8 {
        (self.1 >> 12) as u8 & 0b111
    }
}
