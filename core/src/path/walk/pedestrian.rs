use crate::{
    height::Height,
    path::{Action, Close, Position, Space, Walk},
    rotation::Rotation,
    side::Side,
};

pub struct Pedestrian {
    pub height: Height,
    pub jump_down: Height,
}

impl<S: Space> Walk<S> for Pedestrian {
    fn walk<C>(&self, space: &S, pos: Position, close: &mut C)
    where
        C: Close,
    {
        for action in [Action::LiftUp, Action::LiftDown] {
            if let Some(pos) = self.run(space, pos, action) {
                close.close(action, pos);
            }
        }

        for rotation in [Rotation::Q0, Rotation::Q2, Rotation::Q1, Rotation::Q3] {
            let action = Action::Step(rotation);
            if let Some(pos) = self.run(space, pos, action) {
                close.close(action, pos);
                continue;
            }

            let action = Action::JumpDown(rotation);
            if let Some(pos) = self.run(space, pos, action) {
                close.close(action, pos);
            }
        }
    }

    fn run(&self, space: &S, pos: Position, action: Action) -> Option<Position> {
        let (target, value) = match action {
            Action::Stay => return Some(pos),
            Action::Step(rotation) => {
                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let low = pos.pn.to(rotation) - Height::new(3).unwrap();
                let column = space.column(low, self.height + 6);
                let target = (0..5)
                    .find(|&i| {
                        let pass = column.get(i);
                        (pass.is_passable() || (i < 3 && pass.is_lift()))
                            && (i..self.height.get() as usize + i).all(|k| {
                                let pass = column.get(k + 1);
                                !pass.is_solid()
                            })
                            && match i {
                                0 => space.get(pos.pn.to(Side::Down)).ascent_from(rotation),
                                4 => pass.ascent_from(rotation.opposite()),
                                _ => true,
                            }
                    })
                    .map(|i| low + Height::new(i as u8 + 1).unwrap())?;

                (target, value)
            }
            Action::LiftUp => {
                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let target = space
                    .column(pos.pn, self.height + 1)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_lift(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| pos.pn.to(Side::Up))?;

                (target, value)
            }
            Action::LiftDown => {
                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let target = space
                    .column(pos.pn, self.height + 1)
                    .iter()
                    .enumerate()
                    .all(|(i, pass)| match i {
                        0 => pass.is_lift(),
                        _ => !pass.is_solid(),
                    })
                    .then(|| pos.pn.to(Side::Down))?;

                (target, value)
            }
            Action::JumpDown(rotation) => {
                if !space.get(pos.pn.to(Side::Down)).is_solid() {
                    return None;
                }

                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let low = pos.pn.to(rotation) - (self.jump_down + 1);
                let column = space.column(low, self.height + self.jump_down + 1);
                let target = (0..self.jump_down.get() as usize - 2)
                    .find(|&i| {
                        let pass = column.get(i);
                        pass.is_passable()
                            && (i..self.height.get() as usize + i).all(|k| {
                                let pass = column.get(k + 1);
                                !pass.is_solid()
                            })
                    })
                    .map(|i| low + Height::new(i as u8 + 1).unwrap())?;

                (target, value)
            }
            Action::Fall(_) => todo!(),
            _ => return None,
        };

        Some(Position { pn: target, value })
    }
}
