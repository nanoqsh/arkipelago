use crate::Render;

#[derive(Debug)]
pub enum Control {
    Look(f32, f32),
    Scroll(f32, f32),
    Forward,
    Back,
    Left,
    Right,
}

pub struct Game {}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, ren: &mut Render) {
        ren.draw()
    }

    pub fn input(&mut self, control: Control) {
        println!("{:?}", control)
    }
}
