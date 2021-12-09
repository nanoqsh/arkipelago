use crate::{
    path::{
        action::Action,
        space::Space,
        tree::{NodePtr, Tree},
        walker::{Position, Walker},
    },
    point::Point,
};
use std::collections::{hash_map::Entry, HashMap};

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
            closed: HashMap::default(),
            open: Vec::default(),
            buf: Vec::default(),
            tree: Tree::default(),
        }
    }

    pub fn find<S>(&mut self, pos: Position, walker: Walker, space: &S)
    where
        S: Space,
    {
        if pos.value == 0 {
            return;
        }

        let ptr = self.tree.push(None, (Action::Stay, pos));
        self.open.push(ptr);

        loop {
            for parent in &self.open {
                let pos = self.tree.get(*parent).1;

                walker.from(
                    pos,
                    space,
                    |act, pos| {
                        let ptr = self.tree.push(Some(*parent), (act, pos));
                        if pos.value != 0 {
                            self.buf.push(ptr)
                        }
                    },
                    |pn| self.closed.contains_key(&pn),
                );
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
