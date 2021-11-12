use crate::land::polygon::{Polygon, Polygons};
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
        match self {
            Self::NONE => false,
            Self::FULL => true,
            _ => p(self.0, rhs.0),
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

    pub fn rotated(mut self, rotation: Rotation) -> Self {
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
                self
            }
        }
    }

    pub fn overlaps(&self, rhs: &Self, side: Side, man: &Polygons) -> bool {
        let a = self.get(side);
        let b = rhs.get(side.opposite());
        a.overlaps(b, |a, b| man.eq(a, b))
    }
}
