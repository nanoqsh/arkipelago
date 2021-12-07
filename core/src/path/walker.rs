use crate::{
    height::Height,
    path::{action::Action, space::Space},
    point::Point,
    side::Side,
};

pub(crate) struct Walker {
    height: Height,
    jump_up: Height,
    jump_down: Height,
    jump_over: bool,
    fly: bool,
}

impl Walker {
    fn act<S>(&self, pn: Point, action: Action, space: S) -> Option<Point>
    where
        S: Space,
    {
        let height = self.height + 1;
        match action {
            Action::StepStraight { rotation } => {
                let target = pn.to(rotation);
                space
                    .column(target.to(Side::Down), height)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_solid(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            Action::StepUp { rotation, ascent } => {
                let pn = pn.to(rotation);
                if ascent {
                    let pn = pn.to(Side::Up);
                    space
                        .column(pn, height)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_solid() && pass.ascent_from(rotation.opposite()),
                            _ => !pass.is_solid(),
                        })
                        .then(|| pn.to(Side::Up))
                } else {
                    space
                        .column(pn, height)
                        .iter()
                        .enumerate()
                        .all(|(i, pass)| match i {
                            0 => pass.is_solid(),
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
                    .column(target.to(Side::Down), height)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_solid(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| target)
            }
            _ => todo!(),
        }
    }
}
