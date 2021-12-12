use crate::{
    height::Height,
    path::{Action, Close, Pedestrian, Position, Space, Walk},
    rotation::Rotation,
    side::Side,
};

pub struct Jumper {
    pub walk: Pedestrian,
    pub jump: Height,
}

impl<S: Space> Walk<S> for Jumper {
    fn walk<C>(&self, space: &S, pos: Position, close: &mut C)
    where
        C: Close,
    {
        for rotation in [Rotation::Q0, Rotation::Q2, Rotation::Q1, Rotation::Q3] {
            for action in [Action::JumpUp(rotation), Action::JumpOver(rotation)] {
                if let Some(pos) = self.run(space, pos, action) {
                    close.close(action, pos);
                }
            }
        }

        self.walk.walk(space, pos, close);
    }

    fn run(&self, space: &S, pos: Position, action: Action) -> Option<Position> {
        match action {
            Action::JumpUp(rotation) => {
                let low = pos.pn.to(Side::Down);
                let primary = space.column(low, self.walk.height + self.jump + 1);

                let low = pos.pn.to(rotation);
                let secondary = space.column(low, self.walk.height + self.jump);
                let (target, height) = (2..self.jump.get() as usize)
                    .find(|&i| {
                        let pass = secondary.get(i);
                        pass.is_passable()
                            && (i..self.walk.height.get() as usize + i).all(|k| {
                                let pass = secondary.get(k + 1);
                                !pass.is_solid()
                            })
                            && (i..self.walk.height.get() as usize + i).all(|k| {
                                let pass = primary.get(k + 2);
                                !pass.is_solid()
                            })
                    })
                    .map(|i| (low + Height::new(i as u8 + 1).unwrap(), i as u32))?;

                let cost = height + 1;
                let value = pos.value.checked_sub(cost)?;

                Some(Position { pn: target, value })
            }
            Action::JumpOver(rotation) => {
                let value = pos.value.checked_sub(action.cost().unwrap())?;
                let primary = space.column(pos.pn, self.walk.height);
                let mid = pos.pn.to(rotation);
                let low = mid - Height::new(2).unwrap();
                let seconday = space.column(low, self.walk.height + 2);
                let target = mid.to(rotation);
                let low = target.to(Side::Down);
                let tertiary = space.column(low, self.walk.height + 1);

                (!primary.iter().any(|pass| pass.is_solid())
                    && !seconday.iter().any(|pass| pass.is_solid())
                    && !tertiary.iter().enumerate().any(|(i, pass)| match i {
                        0 => !pass.is_solid(),
                        _ => pass.is_solid(),
                    }))
                .then(|| Position { pn: target, value })
            }
            _ => None,
        }
    }
}
