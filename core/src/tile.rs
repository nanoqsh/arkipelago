use crate::layout::Data;

pub trait Tile {
    fn data(&self, variant: u8) -> &[Data];
}

pub struct Tiles(Vec<Box<dyn Tile>>);

impl Tiles {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self(Vec::default())
    }

    pub fn get(&self, tile: u16) -> &dyn Tile {
        self.0.get(tile as usize).map(Box::as_ref).unwrap()
    }

    pub fn add<T>(&mut self, tile: T) -> u16
    where
        T: Tile + 'static,
    {
        let idx = self.0.len();
        assert!(idx <= u16::MAX as usize);
        self.0.push(Box::new(tile));
        idx as u16
    }
}
