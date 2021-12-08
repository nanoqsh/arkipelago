use crate::{
    path::{
        action::Action,
        space::Space,
        tree::{NodePtr, Tree},
        walker::Walker,
    },
    point::Point,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct PathFinder {
    closed: HashMap<Point, NodePtr>,
    open: Vec<(Point, NodePtr)>,
    tree: Tree<(Action, Point)>,
}

impl PathFinder {
    pub fn new() -> Self {
        Self {
            closed: HashMap::default(),
            open: Vec::default(),
            tree: Tree::default(),
        }
    }

    pub fn find<S>(&mut self, pn: Point, iters: usize, walker: Walker, space: &S)
    where
        S: Space,
    {
        let ptr = self.tree.push(None, (Action::Stay, pn));
        self.open.push((pn, ptr));

        let mut tmp = Vec::new();

        for _ in 0..iters {
            for (front, parent) in &self.open {
                walker.actions_from(*front, space, |act, pn| {
                    if self.closed.contains_key(&pn) {
                        return;
                    }

                    let ptr = self.tree.push(Some(*parent), (act, pn));
                    tmp.push((pn, ptr));
                })
            }

            self.closed.extend(self.open.drain(..));
            self.open.append(&mut tmp);
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
        self.0.closed.keys().copied()
    }

    pub fn to(&self, pn: Point) -> impl Iterator<Item = (Action, Point)> + '_ {
        let ptr = self.0.closed[&pn];
        self.0.tree.list(ptr).map(|node| *node.value())
    }
}
