use super::*;

#[derive(Debug, Eq, PartialEq)]
struct Rec {
    key: String,
    val: i32,
}

impl Rec {
    fn new(key: &str, val: i32) -> Self {
        Rec {
            key: key.to_string(),
            val,
        }
    }
}

impl AsRef<str> for Rec {
    fn as_ref(&self) -> &str {
        self.key.as_str()
    }
}

#[test]
fn new() {
    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key0"),
        (Rec::new("key3", 3), "key1"),
        (Rec::new("key4", 4), "key2"),
    ];
    let root = Rec::new("key0", 0);
    let tree = Tree::from_cmp(root, vals, |val, &key| val.key == key).unwrap();

    let expected = [
        Node {
            parent: 0,
            children: 1..3,
            val: Rec::new("key0", 0),
        },
        Node {
            parent: 0,
            children: 3..4,
            val: Rec::new("key1", 1),
        },
        Node {
            parent: 0,
            children: 4..5,
            val: Rec::new("key2", 2),
        },
        Node {
            parent: 1,
            children: 0..0,
            val: Rec::new("key3", 3),
        },
        Node {
            parent: 2,
            children: 0..0,
            val: Rec::new("key4", 4),
        },
    ];
    assert_eq!(tree.0.as_ref(), expected);
}

#[test]
fn new_failed() {
    let vals = [(Rec::new("key1", 1), "key2")];
    let root = Rec::new("key0", 0);
    let res = Tree::from_cmp(root, vals, |val, &key| val.key == key);
    assert_eq!(res.unwrap_err(), Error::ParentNotFound("key2"));

    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key1"),
        (Rec::new("key3", 3), "key0"),
    ];
    let root = Rec::new("key0", 0);
    let res = Tree::from_cmp(root, vals, |val, &key| val.key == key);
    assert_eq!(res.unwrap_err(), Error::NotAdjacentChildren);
}

#[test]
fn nodes() {
    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key0"),
        (Rec::new("key3", 3), "key2"),
        (Rec::new("key4", 4), "key2"),
    ];
    let root = Rec::new("key0", 0);
    let tree = Tree::new(root, vals).unwrap();
    let nodes: Vec<i32> = tree.nodes().map(|n| (**n).val).collect();
    assert_eq!(nodes, [0, 1, 2, 3, 4]);
}

#[test]
fn nodes_mut() {
    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key0"),
        (Rec::new("key3", 3), "key2"),
        (Rec::new("key4", 4), "key2"),
    ];
    let root = Rec::new("key0", 0);
    let mut tree = Tree::new(root, vals).unwrap();
    let nodes: Vec<i32> = tree.nodes_mut().map(|n| (**n).val).collect();
    assert_eq!(nodes, [0, 1, 2, 3, 4]);
}

#[test]
fn edges() {
    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key0"),
        (Rec::new("key3", 3), "key2"),
        (Rec::new("key4", 4), "key2"),
    ];
    let root = Rec::new("key0", 0);
    let tree = Tree::new(root, vals).unwrap();
    let edges: Vec<(i32, i32)> = tree
        .edges()
        .map(|Edge { parent, child, .. }| ((**parent).val, (**child).val))
        .collect();
    assert_eq!(edges, [(0, 1), (0, 2), (2, 3), (2, 4)]);
}

#[test]
fn edges_mut() {
    let vals = [
        (Rec::new("key1", 1), "key0"),
        (Rec::new("key2", 2), "key0"),
        (Rec::new("key3", 3), "key2"),
        (Rec::new("key4", 4), "key2"),
    ];
    let root = Rec::new("key0", 0);
    let mut tree = Tree::new(root, vals).unwrap();
    let edges: Vec<(i32, i32)> = tree
        .edges_mut()
        .map(|EdgeMut { parent, child, .. }| ((**parent).val, (**child).val))
        .collect();
    assert_eq!(edges, [(0, 1), (0, 2), (2, 3), (2, 4)]);
}
