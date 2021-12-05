use crate::land::polygon::{Axis, Polygon, Polygons};
use core::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Overlay(u16);

impl Overlay {
    const NONE: Self = Self::new_none();
    const FULL: Self = Self::new_full();

    pub const fn new_none() -> Self {
        Self(u16::MAX)
    }

    pub const fn new_full() -> Self {
        Self(u16::MAX - 1)
    }

    pub fn from_polygon(polygon: Polygon, man: &mut Polygons) -> Self {
        Self(man.add(polygon))
    }

    fn overlaps<P>(self, rhs: Self, p: P) -> bool
    where
        P: Fn(u16, u16) -> bool,
    {
        match (self, rhs) {
            (Self::NONE, b) => b == Self::NONE,
            (Self::FULL, _) | (_, Self::NONE) => true,
            (_, Self::FULL) => false,
            (a, b) => p(a.0, b.0),
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Connections([Overlay; 6]);

impl Connections {
    pub fn new() -> Self {
        Self([
            Overlay::new_none(),
            Overlay::new_none(),
            Overlay::new_none(),
            Overlay::new_none(),
            Overlay::new_none(),
            Overlay::new_none(),
        ])
    }

    pub fn set<S>(&mut self, sides: S, overlay: Overlay)
    where
        S: Into<Sides>,
    {
        for side in sides.into() {
            self.0[side as usize] = overlay;
        }
    }

    pub fn get(&self, side: Side) -> Overlay {
        self.0[side as usize]
    }

    pub fn rotated(mut self, rotation: Rotation, man: &mut Polygons) -> Self {
        match rotation {
            Rotation::Q0 => self,
            Rotation::Q1 | Rotation::Q2 | Rotation::Q3 => {
                let left = self.get(Side::Left);
                let right = self.get(Side::Right);
                let forth = self.get(Side::Forth);
                let back = self.get(Side::Back);

                self.set(rotation.rotate(Side::Left), left);
                self.set(rotation.rotate(Side::Right), right);
                self.set(rotation.rotate(Side::Forth), forth);
                self.set(rotation.rotate(Side::Back), back);

                for side in [Side::Up, Side::Down] {
                    match self.get(side) {
                        Overlay::NONE | Overlay::FULL => (),
                        over => self.set(
                            side,
                            Overlay::from_polygon(
                                match man.get(over.0).rotated(rotation) {
                                    Ok(new) => new,
                                    Err(_) => continue,
                                },
                                man,
                            ),
                        ),
                    }
                }

                self
            }
        }
    }

    pub fn overlaps(&self, rhs: &Self, side: Side, man: &Polygons, axis: Axis) -> bool {
        let a = self.get(side);
        let b = rhs.get(side.opposite());
        a.overlaps(b, |a, b| man.eq(a, b, axis))
    }
}

impl Default for Connections {
    fn default() -> Self {
        Self::new()
    }
}
