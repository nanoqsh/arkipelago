use crate::rotation::Rotation;

/// Pass layout.
///
/// Layout: 00abcdls
/// where
///     a: ascent from Q0
///     b: ascent from Q1
///     c: ascent from Q2
///     d: ascent from Q3
///     l: lift / pathless
///     s: solid
#[derive(Copy, Clone)]
pub struct Pass(u8);

impl Pass {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn solid() -> Self {
        Self(1)
    }

    pub const fn lift() -> Self {
        Self(0b10)
    }

    pub const fn pathless() -> Self {
        Self(0b11)
    }

    pub fn ascent<R>(rotations: R) -> Self
    where
        R: IntoIterator<Item = Rotation>,
    {
        let mut b = 1;
        for rotation in rotations {
            b |= 1 << (rotation as u8 + 2);
        }
        Self(b)
    }

    pub const fn is_solid(self) -> bool {
        self.0 & 1 != 0
    }

    pub const fn is_lift(self) -> bool {
        self.0 & 0b11 == 0b10
    }

    pub const fn is_passable(self) -> bool {
        self.0 & 0b11 == 0b01
    }

    pub const fn ascent_from(self, rotation: Rotation) -> bool {
        self.0 & (1 << (rotation as u8 + 2)) != 0
    }
}
