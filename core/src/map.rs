use crate::{chunk::HEIGHT, height::Height, point::ChunkPoints, prelude::*};
use std::collections::HashMap;

pub struct Map<T> {
    chunks: HashMap<ClusterPoint, T>,
}

impl<T> Map<T> {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }

    pub fn get<S>(&self, pn: Point) -> Option<&S>
    where
        T: AsRef<Chunk<S>>,
    {
        let ch = pn.chunk_point();
        let cl = pn.cluster_point();
        let chunk = self.chunk(cl)?;
        Some(chunk.as_ref().get(ch))
    }

    pub fn get_mut<S>(&mut self, pn: Point) -> &mut S
    where
        T: AsMut<Chunk<S>> + Default,
    {
        let ch = pn.chunk_point();
        let cl = pn.cluster_point();
        let chunk = self.chunk_mut(cl);
        chunk.as_mut().get_mut(ch)
    }

    pub fn column<S>(&self, pn: Point, height: Height) -> Option<Column<S>>
    where
        T: AsRef<Chunk<S>>,
    {
        let height = height.get();
        let ch = pn.chunk_point();
        let cl = pn.cluster_point();
        let lo = self.chunk(cl)?;
        let u = ch.y().saturating_add(height);
        if u <= HEIGHT as u8 {
            Some(Column(lo.as_ref().slice(ch, height), &[]))
        } else {
            let hh = u - HEIGHT as u8;
            let lh = height - hh;
            let (x, _, z) = ch.axes();
            let hi = self.chunk(cl.to(Side::Up))?;
            Some(Column(
                lo.as_ref().slice(ch, lh),
                hi.as_ref().slice(ChunkPoint::new(x, 0, z).unwrap(), hh),
            ))
        }
    }

    pub fn column_mut<S>(&mut self, pn: Point, height: Height) -> ColumnMut<S>
    where
        T: AsMut<Chunk<S>> + Default,
    {
        let height = height.get();
        let ch = pn.chunk_point();
        let cl = pn.cluster_point();
        let u = ch.y().saturating_add(height);
        unsafe {
            if u <= HEIGHT as u8 {
                ColumnMut(self.chunk_mut(cl).as_mut().slice_mut(ch, height), &mut [])
            } else {
                let hh = u - HEIGHT as u8;
                let lh = height - hh;
                let up = cl.to(Side::Up);
                self.chunks.entry(cl).or_insert_with(T::default);
                self.chunks.entry(up).or_insert_with(T::default);
                let lo = self.chunks.get_mut(&cl).unwrap() as *mut T;
                let hi = self.chunks.get_mut(&up).unwrap() as *mut T;
                let (x, _, z) = ch.axes();
                ColumnMut(
                    (*lo).as_mut().slice_mut(ch, lh),
                    (*hi)
                        .as_mut()
                        .slice_mut(ChunkPoint::new(x, 0, z).unwrap(), hh),
                )
            }
        }
    }

    pub fn chunk(&self, cl: ClusterPoint) -> Option<&T> {
        self.chunks.get(&cl)
    }

    pub fn chunk_mut(&mut self, cl: ClusterPoint) -> &mut T
    where
        T: Default,
    {
        self.chunks.entry(cl).or_insert_with(T::default)
    }

    pub fn vicinity(&self, cl: ClusterPoint) -> Option<Vicinity<T>> {
        Some(Vicinity {
            chunks: [None; 10],
            center: self.chunk(cl)?,
            map: self,
            cl,
        })
    }

    pub fn iter<S>(&self, cl: ClusterPoint) -> Option<Iter<S>>
    where
        T: AsRef<Chunk<S>>,
    {
        let chunk = self.chunk(cl)?;
        Some(Iter {
            chunk: chunk.as_ref(),
            points: ChunkPoints::new(),
        })
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Column<'a, S>(pub &'a [S], pub &'a [S]);

impl<'a, S> Column<'a, S> {
    pub const fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    pub const fn is_empty(&self) -> bool {
        debug_assert!(!self.0.is_empty());
        false
    }

    pub fn get(&self, idx: usize) -> &'a S {
        if idx < self.0.len() {
            &self.0[idx]
        } else {
            &self.1[idx - self.0.len()]
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a S> {
        self.0.iter().chain(self.1)
    }
}

pub struct ColumnMut<'a, S>(pub &'a mut [S], pub &'a mut [S]);

impl<S> ColumnMut<'_, S> {
    pub const fn len(&self) -> usize {
        self.0.len() + self.1.len()
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut S {
        if idx < self.0.len() {
            &mut self.0[idx]
        } else {
            &mut self.1[idx - self.0.len()]
        }
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut S> {
        self.0.iter_mut().chain(self.1.iter_mut())
    }
}

pub struct Vicinity<'a, T> {
    chunks: [Option<Option<&'a T>>; 10],
    center: &'a T,
    map: &'a Map<T>,
    cl: ClusterPoint,
}

impl<'a, T> Vicinity<'a, T> {
    pub fn center(&self) -> &'a T {
        self.center
    }

    pub fn from(&mut self, side: Side) -> Option<&'a T> {
        self.fetch(side as usize, side)
    }

    pub fn from_upper(&mut self, side: Side) -> Option<&'a T> {
        let idx = match side {
            Side::Left => 0,
            Side::Right => 1,
            Side::Forth => 2,
            Side::Back => 3,
            _ => panic!("wrong side"),
        };

        self.fetch(idx + 6, side)
    }

    fn fetch(&mut self, idx: usize, side: Side) -> Option<&'a T> {
        match &mut self.chunks[idx] {
            Some(from) => *from,
            cell @ None => {
                *cell = Some(self.map.chunk(self.cl.to(side)));
                cell.unwrap()
            }
        }
    }
}

pub struct Iter<'a, S> {
    chunk: &'a Chunk<S>,
    points: ChunkPoints,
}

impl<'a, S: 'a> Iterator for Iter<'a, S> {
    type Item = (&'a S, ChunkPoint);

    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.points.next()?;
        Some((self.chunk.get(ch), ch))
    }
}
