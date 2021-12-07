use crate::{height::Height, map::Column, path::pass::Pass, point::Point};

pub trait Space {
    fn get(&self, pn: Point) -> Pass;

    fn column(&self, pn: Point, height: Height) -> Column<Pass>;
}
