use crate::{
    height::Height,
    path::{action::Action, space::Space},
    point::Point,
    prelude::Rotation,
    side::Side,
};
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Height(u8),
    JumpUp(u8),
    JumpDown(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Height(height) => write!(f, "wrong height {}", height),
            Self::JumpUp(jump) => write!(f, "wrong jump up {}", jump),
            Self::JumpDown(jump) => write!(f, "wrong jump down {}", jump),
        }
    }
}

impl error::Error for Error {}

pub struct Walker {
    height: Height,
    jump_up: Height,
    jump_down: Height,
    jump_over: bool,
    fly: bool,
}

impl Walker {
    const MIN_HEIGHT: u8 = 2;
    const MAX_HEIGHT: u8 = 4;
    const MIN_JUMP: u8 = 1;
    const MAX_JUMP_UP: u8 = 16;
    const MAX_JUMP_DOWN: u8 = 16;

    pub fn new(
        height: u8,
        jump_up: u8,
        jump_down: u8,
        jump_over: bool,
        fly: bool,
    ) -> Result<Self, Error> {
        if !(Self::MIN_HEIGHT..=Self::MAX_HEIGHT).contains(&height) {
            return Err(Error::Height(height));
        }

        if !(Self::MIN_JUMP..=Self::MAX_JUMP_UP).contains(&jump_up) {
            return Err(Error::JumpUp(jump_up));
        }

        if !(Self::MIN_JUMP..=Self::MAX_JUMP_DOWN).contains(&jump_down) {
            return Err(Error::JumpDown(jump_down));
        }

        Ok(Self {
            height: Height::new(height).unwrap(),
            jump_up: Height::new(jump_up).unwrap(),
            jump_down: Height::new(jump_down).unwrap(),
            jump_over,
            fly,
        })
    }
}

impl Walker {
    pub fn actions_from<S, F>(&self, pn: Point, space: &S, mut callback: F)
    where
        S: Space,
        F: FnMut(Action, Point),
    {
        for rotation in [Rotation::Q0, Rotation::Q1, Rotation::Q2, Rotation::Q3] {
            for action in [
                Action::StepStraight { rotation },
                Action::StepUp {
                    rotation,
                    ascent: true,
                },
                Action::StepUp {
                    rotation,
                    ascent: false,
                },
                Action::StepDown {
                    rotation,
                    ascent: true,
                },
                Action::StepDown {
                    rotation,
                    ascent: false,
                },
                Action::JumpUp {
                    rotation,
                    height: Height::new(2).unwrap(),
                },
                Action::JumpDown {
                    rotation,
                    height: Height::new(2).unwrap(),
                },
                Action::JumpOver { rotation },
            ] {
                if let Some(action_pn) = self.act(pn, action, space) {
                    callback(action, action_pn);
                    break;
                }
            }
        }
    }

    pub fn act<S>(&self, pn: Point, action: Action, space: &S) -> Option<Point>
    where
        S: Space,
    {
        let h = self.height + 1;
        match action {
            Action::Stay => Some(pn),
            Action::StepStraight { rotation } => {
                let target = pn.to(rotation);
                space
                    .column(target.to(Side::Down), h)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            Action::StepUp { rotation, ascent } => {
                let pn = pn.to(rotation);
                if ascent {
                    let pn = pn.to(Side::Up);
                    space
                        .column(pn, h)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_passable() && pass.ascent_from(rotation.opposite()),
                            _ => !pass.is_solid(),
                        })
                        .then(|| pn.to(Side::Up))
                } else {
                    space
                        .column(pn, h)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_passable(),
                            _ => !pass.is_solid(),
                        })
                        .then(|| pn.to(Side::Up))
                }
            }
            Action::StepDown { rotation, ascent } => {
                let down = pn.to(Side::Down);
                let mut target = down.to(rotation);
                if ascent {
                    if !space.get(down).ascent_from(rotation.opposite()) {
                        return None;
                    }

                    target = target.to(Side::Down)
                }

                space
                    .column(target.to(Side::Down), h)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            Action::JumpUp {
                rotation,
                height: jump_height,
            } => {
                if jump_height > self.jump_up {
                    return None;
                }

                let pn = pn + jump_height;
                if space
                    .column(pn, self.height)
                    .iter()
                    .any(|pass| pass.is_solid())
                {
                    return None;
                }

                let target = pn.to(rotation);
                space
                    .column(target.to(Side::Down), h)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            Action::JumpDown {
                rotation,
                height: jump_height,
            } => {
                if jump_height > self.jump_down {
                    return None;
                }

                let target = pn.to(rotation) - jump_height;
                space
                    .column(target.to(Side::Down), h + jump_height)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            Action::JumpOver { rotation } => {
                if !self.jump_over {
                    return None;
                }

                let mid = pn.to(rotation);
                let low = mid - Height::new(2).unwrap();
                let target = mid.to(rotation);

                (space.column(low, h + 1).iter().all(|pass| !pass.is_solid())
                    && space
                        .column(target.to(Side::Down), h)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_passable(),
                            _ => !pass.is_solid(),
                        }))
                .then(|| target)
            }
            _ => todo!(),
        }
    }
}
