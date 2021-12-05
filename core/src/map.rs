use crate::{chunk::HEIGHT, point::ChunkPoints, prelude::*};
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

    pub fn get<S>(&self, gl: GlobalPoint) -> Option<&S>
    where
        T: AsRef<Chunk<S>>,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let chunk = self.chunk(cl)?;
        Some(chunk.as_ref().get(ch))
    }

    pub fn get_mut<S>(&mut self, gl: GlobalPoint) -> &mut S
    where
        T: AsMut<Chunk<S>> + Default,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let chunk = self.chunk_mut(cl);
        chunk.as_mut().get_mut(ch)
    }

    pub fn slice<S>(&self, gl: GlobalPoint, height: u8) -> Option<(&[S], &[S])>
    where
        T: AsRef<Chunk<S>>,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let lo = self.chunk(cl)?;
        let u = ch.y().saturating_add(height);
        if u <= HEIGHT as u8 {
            Some((lo.as_ref().slice(ch, height), &[]))
        } else {
            let hh = u - HEIGHT as u8;
            let lh = height - hh;
            let (x, _, z) = ch.axes();
            let hi = self.chunk(cl.to(Side::Up))?;
            Some((
                lo.as_ref().slice(ch, lh),
                hi.as_ref().slice(ChunkPoint::new(x, 0, z).unwrap(), hh),
            ))
        }
    }

    pub fn slice_mut<S>(&mut self, gl: GlobalPoint, height: u8) -> (&mut [S], &mut [S])
    where
        T: AsMut<Chunk<S>> + Default,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let u = ch.y().saturating_add(height);
        unsafe {
            if u <= HEIGHT as u8 {
                (self.chunk_mut(cl).as_mut().slice_mut(ch, height), &mut [])
            } else {
                let hh = u - HEIGHT as u8;
                let lh = height - hh;
                let up = cl.to(Side::Up);
                self.chunks.entry(cl).or_insert_with(T::default);
                self.chunks.entry(up).or_insert_with(T::default);
                let lo = self.chunks.get_mut(&cl).unwrap() as *mut T;
                let hi = self.chunks.get_mut(&up).unwrap() as *mut T;
                let (x, _, z) = ch.axes();
                (
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
