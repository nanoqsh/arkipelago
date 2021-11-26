use crate::{chunk::HEIGHT, prelude::*};
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

    pub fn get_mut<S>(&mut self, gl: GlobalPoint) -> Option<&mut S>
    where
        T: AsMut<Chunk<S>>,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let chunk = self.chunk_mut(cl)?;
        Some(chunk.as_mut().get_mut(ch))
    }

    pub fn slice<S>(&self, gl: GlobalPoint, height: u8) -> Option<(&[S], &[S])>
    where
        T: AsRef<Chunk<S>>,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let lo = self.chunk(cl)?;
        let u = ch.y().saturating_add(height);
        Some(if u <= HEIGHT as u8 {
            (lo.as_ref().slice(ch, height), &[])
        } else {
            let hh = u - HEIGHT as u8;
            let lh = height - hh;
            let (x, _, z) = ch.axes();
            let hi = self.chunk(cl.to(Side::Up))?;
            (
                lo.as_ref().slice(ch, lh),
                hi.as_ref().slice(ChunkPoint::new(x, 0, z).unwrap(), hh),
            )
        })
    }

    pub fn slice_mut<S>(&mut self, gl: GlobalPoint, height: u8) -> Option<(&mut [S], &mut [S])>
    where
        T: AsMut<Chunk<S>>,
    {
        let ch = gl.chunk_point();
        let cl = gl.cluster_point();
        let lo = self.chunk_mut(cl)?;
        let u = ch.y().saturating_add(height);
        Some(if u <= HEIGHT as u8 {
            (lo.as_mut().slice_mut(ch, height), &mut [])
        } else {
            let hh = u - HEIGHT as u8;
            let lh = height - hh;
            let (x, _, z) = ch.axes();
            let hi = self.chunk_mut(cl.to(Side::Up))?;
            (
                lo.as_mut().slice_mut(ch, lh),
                hi.as_mut().slice_mut(ChunkPoint::new(x, 0, z).unwrap(), hh),
            )
        })
    }

    pub fn chunk(&self, cl: ClusterPoint) -> Option<&T> {
        self.chunks.get(&cl)
    }

    pub fn chunk_mut(&mut self, cl: ClusterPoint) -> Option<&mut T> {
        self.chunks.get_mut(&cl)
    }
}

impl<T> Default for Map<T> {
    fn default() -> Self {
        Self::new()
    }
}
