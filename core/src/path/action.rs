use crate::{rotation::Rotation, side::Side};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Stay,
    Step(Rotation),
    LiftUp,
    LiftDown,
    JumpUp(Rotation),
    JumpDown(Rotation),
    JumpOver(Rotation),
    Fall(Rotation),
    Fly(Side),
    Impossible,
}

impl Action {
    pub fn opposite(self) -> Self {
        match self {
            Self::Stay => Self::Stay,
            Self::Step(rotation) => Self::Step(rotation.opposite()),
            Self::LiftUp => Self::LiftDown,
            Self::LiftDown => Self::LiftUp,
            Self::JumpUp(rotation) => Self::JumpDown(rotation.opposite()),
            Self::JumpDown(rotation) => Self::JumpUp(rotation.opposite()),
            Self::JumpOver(rotation) => Self::JumpOver(rotation.opposite()),
            Self::Fall { .. } => Self::Impossible,
            Self::Fly(side) => Self::Fly(side.opposite()),
            Self::Impossible => Self::Impossible,
        }
    }

    pub fn is_final(self) -> bool {
        !matches!(self, Self::LiftUp | Self::LiftDown | Self::Fly(_))
    }

    pub fn cost(self) -> Option<u32> {
        match self {
            Self::Stay => Some(0),
            Self::Step(_) => Some(1),
            Self::LiftUp => Some(1),
            Self::LiftDown => Some(1),
            Self::JumpUp(_) => None,
            Self::JumpDown(_) => Some(2),
            Self::JumpOver(_) => Some(3),
            Self::Fall(_) => Some(1),
            Self::Fly(side) => {
                Some(1 + matches!(side, Side::Left | Side::Right | Side::Forth | Side::Back) as u32)
            }
            Self::Impossible => Some(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constraints() {
        for rotation in [Rotation::Q0, Rotation::Q1, Rotation::Q2, Rotation::Q3] {
            let step = Action::Step(rotation);
            let fly = Action::Fly(rotation.into());
            assert!(step.cost().unwrap() < fly.cost().unwrap());
        }
    }
}
