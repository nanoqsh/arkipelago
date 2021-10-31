mod edges;
mod nodes;
#[cfg(test)]
mod tests;

pub use self::{
    edges::{Edge, EdgeMut, Edges, EdgesMut},
    nodes::{Nodes, NodesMut},
};
use std::{
    error, fmt,
    ops::{Deref, DerefMut, Range},
};

#[derive(Debug, Eq, PartialEq)]
pub enum Error<P> {
    ParentNotFound(P),
    NotAdjacentChildren,
}

impl<P> fmt::Display for Error<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ParentNotFound(_) => write!(f, "parent not found"),
            Self::NotAdjacentChildren => write!(f, "not adjacent children"),
        }
    }
}

impl<P> error::Error for Error<P> where P: fmt::Debug {}

#[derive(Debug, Eq, PartialEq)]
pub struct Node<V> {
    parent: usize,
    children: Range<usize>,
    val: V,
}

impl<V> Deref for Node<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<V> DerefMut for Node<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

#[derive(Debug)]
pub struct Tree<V>(Box<[Node<V>]>);

impl<V> Tree<V> {
    pub fn new<I, P, K>(root: V, vals: I) -> Result<Self, Error<P>>
    where
        I: IntoIterator<Item = (V, P)>,
        V: AsRef<K>,
        P: AsRef<K>,
        K: PartialEq + ?Sized,
    {
        Self::from_cmp(root, vals, |val, key| key.as_ref() == val.as_ref())
    }

    pub fn from_pairs<I>(root: V, pairs: I) -> Result<Self, Error<V>>
    where
        I: IntoIterator<Item = (V, V)>,
        V: PartialEq,
    {
        Self::from_cmp(root, pairs, PartialEq::eq)
    }

    pub fn from_cmp<I, P, F>(root: V, vals: I, mut cmp: F) -> Result<Self, Error<P>>
    where
        I: IntoIterator<Item = (V, P)>,
        F: FnMut(&V, &P) -> bool,
    {
        fn find<T, P>(slice: &[T], mut pred: P) -> Option<Range<usize>>
        where
            P: FnMut(&T) -> bool,
        {
            let mut iter = slice.iter();
            let start = iter.position(&mut pred)?;
            let len = iter.take_while(|&x| pred(x)).count() + 1;
            Some(start..start + len)
        }

        let vals = vals.into_iter();
        let mut nodes = Vec::with_capacity(vals.size_hint().1.unwrap_or(0) + 1);
        nodes.push(Node {
            parent: 0,
            children: 0..0,
            val: root,
        });

        for (val, parent_key) in vals {
            let parent_idx = nodes
                .iter()
                .position(|n| cmp(&n.val, &parent_key))
                .ok_or(Error::ParentNotFound(parent_key))?;

            nodes.push(Node {
                parent: parent_idx,
                children: 0..0,
                val,
            });
        }

        for idx in 0..nodes.len() - 1 {
            let (parent, tail) = nodes[idx..].split_first_mut().unwrap();
            let (start, end) = match find(tail, |n| n.parent == idx) {
                Some(rng) => (rng.start, rng.end),
                None => continue,
            };

            if tail[end..].iter().any(|n| n.parent == idx) {
                return Err(Error::NotAdjacentChildren);
            }

            let offset = idx + 1;
            parent.children = start + offset..end + offset;
        }

        Ok(Self(nodes.into_boxed_slice()))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn nodes(&self) -> Nodes<V> {
        Nodes::new(&self.0)
    }

    pub fn nodes_mut(&mut self) -> NodesMut<V> {
        NodesMut::new(&mut self.0)
    }

    pub fn edges(&self) -> Edges<V> {
        Edges::new(&self.0)
    }

    pub fn edges_mut(&mut self) -> EdgesMut<V> {
        EdgesMut::new(&mut self.0)
    }
}

impl<V> AsRef<[Node<V>]> for Tree<V> {
    fn as_ref(&self) -> &[Node<V>] {
        &self.0
    }
}

impl<V> AsMut<[Node<V>]> for Tree<V> {
    fn as_mut(&mut self) -> &mut [Node<V>] {
        &mut self.0
    }
}
