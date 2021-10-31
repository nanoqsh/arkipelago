use crate::Node;
use std::{marker::PhantomData, ops::Range};

pub struct Edge<'a, V> {
    pub parent: &'a Node<V>,
    pub child: &'a Node<V>,
    pub parent_idx: usize,
    pub child_idx: usize,
}

pub struct Edges<'a, V> {
    tree: Range<*const Node<V>>,
    parent_idx: usize,
    child_idx: usize,
    lt: PhantomData<&'a V>,
}

impl<'a, V> Edges<'a, V> {
    pub(crate) fn new(nodes: &[Node<V>]) -> Self {
        Self {
            tree: nodes.as_ptr_range(),
            parent_idx: 0,
            child_idx: 0,
            lt: PhantomData,
        }
    }

    fn next(&mut self) -> Option<Edge<'a, V>> {
        loop {
            let start_ptr = self.tree.start;
            let end_ptr = self.tree.end;
            let parent_idx = self.parent_idx;

            return unsafe {
                let parent_ptr = start_ptr.add(parent_idx);
                if parent_ptr == end_ptr {
                    return None;
                }

                let parent = &*parent_ptr;
                let child_idx = parent.children.start + self.child_idx;
                debug_assert_ne!(parent_idx, child_idx);

                if !parent.children.contains(&child_idx) {
                    self.child_idx = 0;
                    self.parent_idx += 1;
                    continue;
                }

                self.child_idx += 1;
                let child_ptr = start_ptr.add(child_idx);
                let child = &*child_ptr;
                Some(Edge {
                    parent,
                    child,
                    parent_idx,
                    child_idx,
                })
            };
        }
    }
}

impl<'a, V> Iterator for Edges<'a, V> {
    type Item = Edge<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

pub struct EdgeMut<'a, V> {
    pub parent: &'a mut Node<V>,
    pub child: &'a mut Node<V>,
    pub parent_idx: usize,
    pub child_idx: usize,
}

pub struct EdgesMut<'a, V> {
    tree: Range<*mut Node<V>>,
    parent_idx: usize,
    child_idx: usize,
    lt: PhantomData<&'a mut V>,
}

impl<'a, V> EdgesMut<'a, V> {
    pub(crate) fn new(nodes: &mut [Node<V>]) -> Self {
        Self {
            tree: nodes.as_mut_ptr_range(),
            parent_idx: 0,
            child_idx: 0,
            lt: PhantomData,
        }
    }

    fn next(&mut self) -> Option<EdgeMut<'a, V>> {
        loop {
            let start_ptr = self.tree.start;
            let end_ptr = self.tree.end;
            let parent_idx = self.parent_idx;

            return unsafe {
                let parent_ptr = start_ptr.add(parent_idx);
                if parent_ptr == end_ptr {
                    return None;
                }

                let parent = &mut *parent_ptr;
                let child_idx = parent.children.start + self.child_idx;
                debug_assert_ne!(parent_idx, child_idx);

                if !parent.children.contains(&child_idx) {
                    self.child_idx = 0;
                    self.parent_idx += 1;
                    continue;
                }

                self.child_idx += 1;
                let child_ptr = start_ptr.add(child_idx);
                let child = &mut *child_ptr;
                Some(EdgeMut {
                    parent,
                    child,
                    parent_idx,
                    child_idx,
                })
            };
        }
    }
}

impl<'a, V> Iterator for EdgesMut<'a, V> {
    type Item = EdgeMut<'a, V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
