use crate::Node;
use std::{marker::PhantomData, ops::Range};

pub struct Nodes<'a, V> {
    tree: Range<*const Node<V>>,
    node_idx: usize,
    lt: PhantomData<&'a V>,
}

impl<'a, V> Nodes<'a, V> {
    pub(crate) fn new(nodes: &[Node<V>]) -> Self {
        Self {
            tree: nodes.as_ptr_range(),
            node_idx: 0,
            lt: PhantomData,
        }
    }

    fn next(&mut self) -> Option<&'a Node<V>> {
        let start_ptr = self.tree.start;
        let end_ptr = self.tree.end;

        unsafe {
            let node_ptr = start_ptr.add(self.node_idx);
            if node_ptr == end_ptr {
                return None;
            }

            self.node_idx += 1;
            let node = &*node_ptr;
            Some(node)
        }
    }
}

impl<'a, V> Iterator for Nodes<'a, V> {
    type Item = &'a Node<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

pub struct NodesMut<'a, V> {
    tree: Range<*mut Node<V>>,
    node_idx: usize,
    lt: PhantomData<&'a mut V>,
}

impl<'a, V> NodesMut<'a, V> {
    pub(crate) fn new(nodes: &mut [Node<V>]) -> Self {
        Self {
            tree: nodes.as_mut_ptr_range(),
            node_idx: 0,
            lt: PhantomData,
        }
    }

    fn next(&mut self) -> Option<&'a mut Node<V>> {
        let start_ptr = self.tree.start;
        let end_ptr = self.tree.end;

        unsafe {
            let node_ptr = start_ptr.add(self.node_idx);
            if node_ptr == end_ptr {
                return None;
            }

            self.node_idx += 1;
            let node = &mut *node_ptr;
            Some(node)
        }
    }
}

impl<'a, V> Iterator for NodesMut<'a, V> {
    type Item = &'a mut Node<V>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
