use crate::prelude::*;

pub(crate) const SIDE: usize = 16;
pub(crate) const HEIGHT: usize = SIDE * 2;

pub struct Chunk<T>([[[T; HEIGHT]; SIDE]; SIDE]);

impl<T> Chunk<T> {
    pub fn get(&self, point: ChunkPoint) -> &T {
        let (x, y, z) = point.axes();
        unsafe { self.get_unchecked(x as usize, y as usize, z as usize) }
    }

    pub fn get_mut(&mut self, point: ChunkPoint) -> &mut T {
        let (x, y, z) = point.axes();
        unsafe { self.get_unchecked_mut(x as usize, y as usize, z as usize) }
    }

    unsafe fn get_unchecked(&self, x: usize, y: usize, z: usize) -> &T {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0.get_unchecked(y).get_unchecked(x).get_unchecked(z)
    }

    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0
            .get_unchecked_mut(y)
            .get_unchecked_mut(x)
            .get_unchecked_mut(z)
    }
}
