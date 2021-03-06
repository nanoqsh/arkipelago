use core::prelude::{Height, TileIndex, VariantIndex};

/// Slab layout.
///
/// Can be a empty, base or truck.
/// Empty -> all bits are 0.
/// Base -> tttttttt_tttttttt_00000hhh_vvvvvvvv
/// where
///     t: tile
///     h: height (actual height - 1)
///     v: variant
/// Trunk -> tttttttt_tttttttt_lllodddd_dddddddd
/// where
///     t: tile
///     o: obj
///     l: level (always > 0)
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
            Self(a, b) if b >> 13 & 0b111 == 0 => Typed::Base(Base(a, b)),
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

#[derive(Copy, Clone)]
pub(crate) struct Base(u16, u16);

impl Base {
    pub const fn new(tile: TileIndex, variant: VariantIndex, height: Height) -> Self {
        Self(
            tile.get(),
            variant.get() as u16 | ((height.get() - 1) as u16) << 8,
        )
    }

    pub fn tile(self) -> TileIndex {
        TileIndex::new(self.0).unwrap()
    }

    pub const fn variant(self) -> VariantIndex {
        VariantIndex(self.1 as u8)
    }

    pub fn height(self) -> Height {
        Height::new(((self.1 >> 8) as u8 & 0b111) + 1).unwrap()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Trunk(u16, u16);

impl Trunk {
    pub const fn new(tile: TileIndex, data: u16, obj: bool, level: u8) -> Self {
        let mut b = data | (level as u16) << 13;
        if obj {
            b |= 1 << 12;
        }
        Self(tile.get(), b)
    }

    pub const fn is_obj(self) -> bool {
        self.1 & 1 << 12 != 0
    }

    pub const fn data(self) -> u16 {
        self.1 & 0b111111111111
    }

    pub const fn level(self) -> u8 {
        (self.1 >> 13) as u8 & 0b111
    }

    pub fn set_data(&mut self, data: u16) {
        self.1 |= data;
    }
}
