use crate::{height::Height, point::Point, rotation::Rotation, side::Side};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Action {
    Stay,
    StepStraight { rotation: Rotation },
    StepUp { rotation: Rotation, ascent: bool },
    StepDown { rotation: Rotation, ascent: bool },
    LiftUp,
    LiftDown,
    JumpUp { rotation: Rotation, height: Height },
    JumpDown { rotation: Rotation, height: Height },
    JumpOver { rotation: Rotation },
    Fall { rotation: Rotation },
    Fly { side: Side },
    Impossible,
}

impl Action {
    pub fn opposite(self) -> Self {
        match self {
            Self::Stay => Self::Stay,
            Self::StepStraight { rotation } => Self::StepStraight {
                rotation: rotation.opposite(),
            },
            Self::StepUp { rotation, ascent } => Self::StepDown {
                rotation: rotation.opposite(),
                ascent,
            },
            Self::StepDown { rotation, ascent } => Self::StepUp {
                rotation: rotation.opposite(),
                ascent,
            },
            Self::LiftUp => Self::LiftDown,
            Self::LiftDown => Self::LiftUp,
            Self::JumpUp { rotation, height } => Self::JumpDown {
                rotation: rotation.opposite(),
                height,
            },
            Self::JumpDown { rotation, height } => Self::JumpUp {
                rotation: rotation.opposite(),
                height,
            },
            Self::JumpOver { rotation } => Self::JumpOver {
                rotation: rotation.opposite(),
            },
            Self::Fall { .. } => Self::Impossible,
            Self::Fly { side } => Self::Fly {
                side: side.opposite(),
            },
            Self::Impossible => Self::Impossible,
        }
    }

    pub fn is_final(self) -> bool {
        !matches!(self, Action::Fly { .. })
    }

    pub fn cost(self) -> u32 {
        match self {
            Self::Stay => 0,
            Self::StepStraight { .. } => 1,
            Self::StepUp { .. } => 1,
            Self::StepDown { .. } => 1,
            Self::LiftUp => 1,
            Self::LiftDown => 1,
            Self::JumpUp { height, .. } => height.get() as u32 + 1,
            Self::JumpDown { height, .. } => height.get() as u32,
            Self::JumpOver { .. } => 3,
            Self::Fall { .. } => 1,
            Self::Fly { .. } => 1,
            Self::Impossible => 0,
        }
    }

    pub fn target(self, src: Point) -> Point {
        match self {
            Self::Stay => src,
            Self::StepStraight { rotation } => src.to(rotation),
            Self::StepUp { rotation, ascent } => {
                let dst = src.to(rotation).to(Side::Up);
                if ascent {
                    dst.to(Side::Up)
                } else {
                    dst
                }
            }
            Self::StepDown { rotation, ascent } => {
                let dst = src.to(Side::Down).to(rotation);
                if ascent {
                    dst.to(Side::Down)
                } else {
                    dst
                }
            }
            Self::LiftUp => src.to(Side::Up),
            Self::LiftDown => src.to(Side::Down),
            Self::JumpUp { rotation, height } => (src + height).to(rotation),
            Self::JumpDown { rotation, height } => (src - height).to(rotation),
            Self::JumpOver { rotation } => src.to(rotation).to(rotation),
            Self::Fall { rotation } => src.to(rotation),
            Self::Fly { side } => src.to(side),
            Self::Impossible => src,
        }
    }
}
