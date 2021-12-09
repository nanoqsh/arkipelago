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

#[derive(Copy, Clone)]
pub struct Position {
    pub pn: Point,
    pub value: u32,
}

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
    const MAX_JUMP_UP: u8 = 8;
    const MAX_JUMP_DOWN: u8 = 8;

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
    pub(crate) fn from<S, F, C>(&self, pos: Position, space: &S, mut callback: F, closed: C)
    where
        S: Space,
        F: FnMut(Action, Position),
        C: Fn(Point) -> bool,
    {
        'out: for rotation in [Rotation::Q0, Rotation::Q2, Rotation::Q1, Rotation::Q3] {
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
                Action::Fly {
                    side: rotation.into(),
                },
            ] {
                if closed(action.target(pos.pn)) {
                    break;
                }

                if let Some(pos) = self.act(pos, action, space) {
                    callback(action, pos);
                    continue 'out;
                }
            }

            for jump in 2..=self.jump_up.get() {
                let action = Action::JumpUp {
                    rotation,
                    height: Height::new(jump).unwrap(),
                };

                if closed(action.target(pos.pn)) {
                    break;
                }

                if let Some(pos) = self.act(pos, action, space) {
                    callback(action, pos);
                    break;
                }
            }

            for jump in 2..=self.jump_down.get() {
                let action = Action::JumpDown {
                    rotation,
                    height: Height::new(jump).unwrap(),
                };

                if closed(action.target(pos.pn)) {
                    break;
                }

                if let Some(pos) = self.act(pos, action, space) {
                    callback(action, pos);
                    break;
                }
            }
        }

        for action in [Action::LiftUp, Action::Fly { side: Side::Up }] {
            if closed(action.target(pos.pn)) {
                break;
            }

            if let Some(pos) = self.act(pos, action, space) {
                callback(action, pos);
                break;
            }
        }

        for action in [Action::LiftDown, Action::Fly { side: Side::Down }] {
            if closed(action.target(pos.pn)) {
                break;
            }

            if let Some(pos) = self.act(pos, action, space) {
                callback(action, pos);
                break;
            }
        }
    }

    pub fn act<S>(&self, pos: Position, action: Action, space: &S) -> Option<Position>
    where
        S: Space,
    {
        let value = if pos.value < action.cost() {
            return None;
        } else {
            pos.value - action.cost()
        };

        let h = self.height + 1;
        let target = action.target(pos.pn);
        let ok = match action {
            Action::Stay => true,
            Action::StepStraight { .. } => space
                .column(target.to(Side::Down), h)
                .iter()
                .enumerate()
                .all(|(i, pass)| match i {
                    0 => pass.is_passable(),
                    1 => !pass.is_solid() || pass.is_lift(),
                    _ => !pass.is_solid(),
                }),
            Action::StepUp { rotation, ascent } => {
                let mut it = space.column(target.to(Side::Down), h).iter().enumerate();

                if ascent {
                    it.all(|(i, pass)| match i {
                        0 => pass.is_passable() && pass.ascent_from(rotation.opposite()),
                        _ => !pass.is_solid(),
                    })
                } else {
                    it.all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
                }
            }
            Action::StepDown { rotation, ascent } => {
                if ascent
                    && !space
                        .get(pos.pn.to(Side::Down))
                        .ascent_from(rotation.opposite())
                {
                    return None;
                }

                space
                    .column(target.to(Side::Down), h)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
            }
            Action::LiftUp => space
                .column(target.to(Side::Down), self.height)
                .iter()
                .enumerate()
                .all(|(i, pass)| match i {
                    0 => pass.is_lift(),
                    _ => !pass.is_solid(),
                }),
            Action::LiftDown => space
                .column(target.to(Side::Up), self.height)
                .iter()
                .enumerate()
                .all(|(i, pass)| match i {
                    0 => pass.is_lift(),
                    _ => !pass.is_solid(),
                }),
            Action::JumpUp {
                height: jump_height,
                ..
            } => {
                if jump_height > self.jump_up {
                    return None;
                }

                if space
                    .column(pos.pn + jump_height, self.height)
                    .iter()
                    .any(|pass| pass.is_solid())
                {
                    return None;
                }

                space
                    .column(target.to(Side::Down), h)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
            }
            Action::JumpDown {
                height: jump_height,
                ..
            } => {
                if jump_height > self.jump_down {
                    return None;
                }

                space
                    .column(target.to(Side::Down), h + jump_height)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_passable(),
                        _ => !pass.is_solid(),
                    })
            }
            Action::JumpOver { rotation } => {
                if !self.jump_over {
                    return None;
                }

                let low = pos.pn.to(rotation) - Height::new(2).unwrap();
                space.column(low, h + 1).iter().all(|pass| !pass.is_solid())
                    && space
                        .column(target.to(Side::Down), h)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_passable(),
                            _ => !pass.is_solid(),
                        })
            }
            Action::Fall { .. } => todo!(),
            Action::Fly { .. } => {
                if !self.fly {
                    return None;
                }

                space
                    .column(target, self.height)
                    .iter()
                    .all(|pass| !pass.is_solid())
            }
            Action::Impossible => return None,
        };

        ok.then(|| Position { pn: target, value })
    }
}
