use crate::{
    path::{
        action::Action,
        space::Space,
        tree::{NodePtr, Tree},
        walk::{Close, Position, Walk},
    },
    point::Point,
};
use fxhash::{FxBuildHasher, FxHashMap as HashMap};
use std::collections::hash_map::Entry;

#[derive(Default)]
pub struct PathFinder {
    closed: HashMap<Point, NodePtr>,
    open: Vec<NodePtr>,
    buf: Vec<NodePtr>,
    tree: Tree<(Action, Position)>,
}

impl PathFinder {
    pub fn new() -> Self {
        Self {
            closed: HashMap::with_capacity_and_hasher(64, FxBuildHasher::default()),
            open: Vec::with_capacity(64),
            buf: Vec::with_capacity(64),
            tree: Tree::default(),
        }
    }

    pub fn find<W, S>(&mut self, pos: Position, walk: &W, space: &S)
    where
        W: Walk<S>,
        S: Space,
    {
        if pos.value == 0 {
            return;
        }

        let ptr = self.tree.push(NodePtr::ROOT, (Action::Stay, pos));
        self.open.push(ptr);

        loop {
            for parent in &self.open {
                let pos = self.tree.get(*parent).1;
                let mut close = Closer {
                    closed: &self.closed,
                    buf: &mut self.buf,
                    tree: &mut self.tree,
                    parent: *parent,
                };

                walk.walk(space, pos, &mut close)
            }

            for ptr in self.open.drain(..) {
                let node = self.tree.get(ptr);
                match self.closed.entry(node.1.pn) {
                    Entry::Occupied(mut en) => {
                        let old = self.tree.get(*en.get());
                        if old.1.value < node.1.value {
                            en.insert(ptr);
                        }
                    }
                    Entry::Vacant(en) => {
                        en.insert(ptr);
                    }
                }
            }

            if self.buf.is_empty() {
                break;
            }

            self.open.append(&mut self.buf);
        }
    }

    pub fn path(&self) -> Path {
        Path(self)
    }

    pub fn clear(&mut self) {
        self.closed.clear();
        self.open.clear();
        self.tree.clear();
    }
}

struct Closer<'a> {
    closed: &'a HashMap<Point, NodePtr>,
    buf: &'a mut Vec<NodePtr>,
    tree: &'a mut Tree<(Action, Position)>,
    parent: NodePtr,
}

impl Close for Closer<'_> {
    fn close(&mut self, action: Action, pos: Position) {
        if self.closed.contains_key(&pos.pn) {
            return;
        }

        let ptr = self.tree.push(self.parent, (action, pos));
        if pos.value != 0 {
            self.buf.push(ptr)
        }
    }
}

pub struct Path<'a>(&'a PathFinder);

impl Path<'_> {
    pub fn points(&self) -> impl Iterator<Item = Point> + '_ {
        self.0
            .closed
            .iter()
            .filter_map(|(pn, ptr)| self.0.tree.get(*ptr).0.is_final().then(|| *pn))
    }

    pub fn to(&self, pn: Point) -> impl Iterator<Item = (Action, Point)> + '_ {
        let ptr = self.0.closed[&pn];
        self.0.tree.list(ptr).map(|node| {
            let (action, pos) = node.value();
            (*action, pos.pn)
        })
    }
}
