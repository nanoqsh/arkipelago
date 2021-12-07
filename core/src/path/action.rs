use crate::rotation::Rotation;

pub(crate) enum Action {
    StepStraight { rotation: Rotation },
    StepUp { rotation: Rotation, ascent: bool },
    StepDown { rotation: Rotation, ascent: bool },
    JumpUp { rotation: Rotation, height: u8 },
    JumpDown { rotation: Rotation, height: u8 },
    JumpOver { rotation: Rotation },
    LiftUp,
    LiftDown,
    FlyUp,
    FlyDown,
}
