use crate::pass::*;

pub trait Draw<S> {
    fn draw<'a>(&self, pass: Pass<'a, S>)
    where
        S: Stage<'a>;
}

pub trait Pipe {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>);
}

#[derive(Default)]
pub struct Pipeline<'a> {
    solid: Vec<&'a dyn Draw<Solid>>,
    skin: Vec<&'a dyn Draw<Skin>>,
    color: Vec<&'a dyn Draw<Color>>,
    interface: Vec<&'a dyn Draw<Interface>>,
}

impl<'a> Pipeline<'a> {
    pub fn push_solid(&mut self, draw: &'a dyn Draw<Solid>) {
        self.solid.push(draw)
    }

    pub fn push_skin(&mut self, draw: &'a dyn Draw<Skin>) {
        self.skin.push(draw)
    }

    pub fn push_color(&mut self, draw: &'a dyn Draw<Color>) {
        self.color.push(draw)
    }

    pub fn push_interface(&mut self, draw: &'a dyn Draw<Interface>, order: u32) {
        let _ = order;
        self.interface.push(draw)
    }

    pub fn fill<D>(&mut self, draws: D)
    where
        D: IntoIterator<Item = &'a dyn Pipe>,
    {
        for draw in draws {
            draw.pipe(self)
        }
    }

    pub fn clear(&mut self) {
        self.solid.clear();
        self.skin.clear();
        self.color.clear();
        self.interface.clear();
    }

    pub(crate) fn draw_solid(&self, inner: SolidInner) {
        for draw in &self.solid {
            draw.draw(Pass(inner))
        }
    }

    pub(crate) fn draw_skin(&self, inner: SkinInner) {
        for draw in &self.skin {
            draw.draw(Pass(inner))
        }
    }

    pub(crate) fn draw_color(&self, inner: ColorInner) {
        for draw in &self.color {
            draw.draw(Pass(inner))
        }
    }

    pub(crate) fn draw_interface(&self, inner: InterfaceInner) {
        for draw in &self.interface {
            draw.draw(Pass(inner))
        }
    }
}
