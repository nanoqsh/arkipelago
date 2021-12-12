use crate::{
    path::{Action, Space},
    point::Point,
};

pub trait Close {
    fn close(&mut self, action: Action, pos: Position);
}

pub trait Walk<S: Space> {
    fn walk<C>(&self, space: &S, pos: Position, close: &mut C)
    where
        C: Close;

    fn run(&self, space: &S, pos: Position, action: Action) -> Option<Position>;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub pn: Point,
    pub value: u32,
}
