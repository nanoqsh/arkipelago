pub struct Layout {}

pub trait Tile {
    fn layout(&self) -> Layout;
}
