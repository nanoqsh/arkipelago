use crate::{height::Height, rotation::Rotation};

#[derive(Copy, Clone)]
pub enum Action {
    Stay,
    StepStraight { rotation: Rotation },
    StepUp { rotation: Rotation, ascent: bool },
    StepDown { rotation: Rotation, ascent: bool },
    JumpUp { rotation: Rotation, height: Height },
    JumpDown { rotation: Rotation, height: Height },
    JumpOver { rotation: Rotation },
    LiftUp,
    LiftDown,
    FlyUp,
    FlyDown,
}
