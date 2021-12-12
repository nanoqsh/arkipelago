use crate::{
    path::{Action, Close, Pedestrian, Position, Space, Walk},
    side::{Side, Sides},
};

struct FlyerCloser<'a, C> {
    close: &'a mut C,
    closed_sides: Sides,
}

impl<C: Close> Close for FlyerCloser<'_, C> {
    fn close(&mut self, action: Action, pos: Position) {
        let action = match action {
            Action::LiftUp => {
                let side = Side::Up;
                self.closed_sides |= side;
                Action::Fly(side)
            }
            Action::LiftDown => {
                let side = Side::Down;
                self.closed_sides |= side;
                Action::Fly(side)
            }
            Action::Step(rotation) => {
                let side: Side = rotation.into();
                self.closed_sides |= side;
                action
            }
            _ => action,
        };

        self.close.close(action, pos)
    }
}

pub struct Flyer {
    pub walk: Pedestrian,
}

impl<S: Space> Walk<S> for Flyer {
    fn walk<C>(&self, space: &S, pos: Position, close: &mut C)
    where
        C: Close,
    {
        let mut close = FlyerCloser {
            close,
            closed_sides: Sides::empty(),
        };

        self.walk.walk(space, pos, &mut close);

        for open_side in !close.closed_sides {
            let action = Action::Fly(open_side);
            if let Some(pos) = self.run(space, pos, action) {
                close.close.close(action, pos);
            }
        }
    }

    fn run(&self, space: &S, pos: Position, action: Action) -> Option<Position> {
        match action {
            Action::Fly(side) => {
                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let target = pos.pn.to(side);
                space
                    .column(target, self.walk.height)
                    .iter()
                    .all(|pass| !pass.is_solid())
                    .then(|| Position { pn: target, value })
            }
            _ => None,
        }
    }
}
