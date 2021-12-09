use crate::{
    height::Height,
    path::{action::Action, space::Space},
    point::Point,
    prelude::Rotation,
    side::Side,
};

#[derive(Copy, Clone)]
pub struct Position {
    pub pn: Point,
    pub value: u32,
}

pub(crate) fn from<W, S, F, C>(walk: &W, pos: Position, space: &S, mut callback: F, closed: C)
where
    W: Walk<S>,
    S: Space,
    F: FnMut(Action, Position),
    C: Fn(Point) -> bool,
{
    for rotation in [Rotation::Q0, Rotation::Q2, Rotation::Q1, Rotation::Q3] {
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
            let target = action.target(pos.pn);
            if closed(target) {
                break;
            }

            if let Some(value) = walk.walk(pos, target, action, space) {
                callback(action, Position { pn: target, value });
                break;
            }
        }
    }

    for action in [Action::Fly { side: Side::Up }, Action::LiftUp] {
        let target = action.target(pos.pn);
        if closed(target) {
            break;
        }

        if let Some(value) = walk.walk(pos, target, action, space) {
            callback(action, Position { pn: target, value });
            break;
        }
    }

    for action in [Action::Fly { side: Side::Down }, Action::LiftDown] {
        let target = action.target(pos.pn);
        if closed(target) {
            break;
        }

        if let Some(value) = walk.walk(pos, target, action, space) {
            callback(action, Position { pn: target, value });
            break;
        }
    }
}

pub(crate) trait Walk<S: Space> {
    fn walk(&self, pos: Position, target: Point, action: Action, space: &S) -> Option<u32>;
}

pub struct Pedestrian {
    pub height: Height,
}

impl<S: Space> Walk<S> for Pedestrian {
    fn walk(&self, pos: Position, target: Point, action: Action, space: &S) -> Option<u32> {
        let cost = action.cost();
        let value = if pos.value < cost {
            return None;
        } else {
            pos.value - cost
        };

        let h = self.height + 1;
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
            Action::Fall { .. } => todo!(),
            _ => return None,
        };

        ok.then(|| value)
    }
}

pub struct Flyer {
    pub walk: Pedestrian,
}

impl<S: Space> Walk<S> for Flyer {
    fn walk(&self, pos: Position, target: Point, action: Action, space: &S) -> Option<u32> {
        match action {
            Action::Fly { .. } => {
                let cost = action.cost();
                let value = if pos.value < cost {
                    return None;
                } else {
                    pos.value - cost
                };

                space
                    .column(target, self.walk.height)
                    .iter()
                    .all(|pass| !pass.is_solid())
                    .then(|| value)
            }
            _ => self.walk.walk(pos, target, action, space),
        }
    }
}
