use crate::prelude::*;

pub(crate) const SIDE: usize = 16;
pub(crate) const HEIGHT: usize = SIDE * 2;

pub struct Chunk<T>([[[T; HEIGHT]; SIDE]; SIDE]);

impl<T> Chunk<T> {
    pub fn filled(val: T) -> Self
    where
        T: Copy,
    {
        Self([[[val; HEIGHT]; SIDE]; SIDE])
    }

    pub fn get(&self, point: ChunkPoint) -> &T {
        let (x, y, z) = point.axes();
        unsafe { self.get_unchecked(x as usize, y as usize, z as usize) }
    }

    pub fn get_mut(&mut self, point: ChunkPoint) -> &mut T {
        let (x, y, z) = point.axes();
        unsafe { self.get_unchecked_mut(x as usize, y as usize, z as usize) }
    }

    pub fn slice(&self, point: ChunkPoint, height: u8) -> &[T] {
        let (x, y, z) = point.axes();
        let u = y.saturating_add(height);
        assert!(u <= HEIGHT as u8);
        unsafe { self.slice_unchecked(x as usize, y as usize, z as usize, u as usize) }
    }

    pub fn slice_mut(&mut self, point: ChunkPoint, height: u8) -> &mut [T] {
        let (x, y, z) = point.axes();
        let u = y.saturating_add(height);
        assert!(u <= HEIGHT as u8);
        unsafe { self.slice_unchecked_mut(x as usize, y as usize, z as usize, u as usize) }
    }

    unsafe fn get_unchecked(&self, x: usize, y: usize, z: usize) -> &T {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0.get_unchecked(z).get_unchecked(x).get_unchecked(y)
    }

    unsafe fn get_unchecked_mut(&mut self, x: usize, y: usize, z: usize) -> &mut T {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0
            .get_unchecked_mut(z)
            .get_unchecked_mut(x)
            .get_unchecked_mut(y)
    }

    unsafe fn slice_unchecked(&self, x: usize, y: usize, z: usize, u: usize) -> &[T] {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0.get_unchecked(z).get_unchecked(x).get_unchecked(y..u)
    }

    unsafe fn slice_unchecked_mut(&mut self, x: usize, y: usize, z: usize, u: usize) -> &mut [T] {
        debug_assert!(x < SIDE);
        debug_assert!(y < HEIGHT);
        debug_assert!(z < SIDE);
        self.0
            .get_unchecked_mut(z)
            .get_unchecked_mut(x)
            .get_unchecked_mut(y..u)
    }
}

impl<T: Copy + Default> Default for Chunk<T> {
    fn default() -> Self {
        Self::filled(T::default())
    }
}

impl<T> AsRef<Chunk<T>> for Chunk<T> {
    fn as_ref(&self) -> &Chunk<T> {
        self
    }
}

impl<T> AsMut<Chunk<T>> for Chunk<T> {
    fn as_mut(&mut self) -> &mut Chunk<T> {
        self
    }
}
