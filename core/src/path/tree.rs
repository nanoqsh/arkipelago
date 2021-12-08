use std::ops;

#[derive(Copy, Clone)]
pub(crate) struct NodePtr(u32);

pub(crate) struct Node<T> {
    parent: Option<NodePtr>,
    val: T,
}

impl<T> Node<T> {
    pub fn value(&self) -> &T {
        &self.val
    }
}

impl<T> ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

pub(crate) struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::default(),
        }
    }

    pub fn push(&mut self, parent: Option<NodePtr>, val: T) -> NodePtr {
        let len = self.nodes.len() as u32;
        assert!(parent.map(|p| p.0 < len).unwrap_or(true));
        self.nodes.push(Node { parent, val });
        NodePtr(len)
    }

    pub fn get(&self, ptr: NodePtr) -> &Node<T> {
        &self.nodes[ptr.0 as usize]
    }

    pub fn list(&self, ptr: NodePtr) -> List<T> {
        List {
            tree: self,
            ptr: Some(ptr),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear()
    }
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct List<'a, T> {
    tree: &'a Tree<T>,
    ptr: Option<NodePtr>,
}

impl<'a, T> List<'a, T> {
    pub fn node(&self) -> Option<&'a Node<T>> {
        Some(self.tree.get(self.ptr?))
    }

    pub fn next(&mut self) {
        if let Some(node) = self.node() {
            self.ptr = node.parent;
        }
    }
}

impl<'a, T> Iterator for List<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node()?;
        self.next();
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut tree = Tree::new();
        let i0 = tree.push(None, 0);
        let i1 = tree.push(Some(i0), 1);
        let i2 = tree.push(Some(i0), 2);

        assert_eq!(tree.get(i0).value(), &0);
        assert_eq!(tree.get(i1).value(), &1);
        assert_eq!(tree.get(i2).value(), &2);
    }

    #[test]
    fn list() {
        let mut tree = Tree::new();
        let i0 = tree.push(None, 0);
        let i1 = tree.push(Some(i0), 1);
        let i2 = tree.push(Some(i0), 2);

        let vals: Vec<_> = tree.list(i1).into_iter().map(Node::value).collect();
        assert_eq!(vals, [&1, &0]);
        let vals: Vec<_> = tree.list(i2).into_iter().map(Node::value).collect();
        assert_eq!(vals, [&2, &0]);
    }
}
