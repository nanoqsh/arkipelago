use crate::pass::*;

pub trait Draw<S> {
    fn draw<'a>(&self, pass: Pass<'a, S>)
    where
        S: Stage<'a>;
}

pub trait Pipe {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>);
}

impl<T: Pipe + ?Sized> Pipe for &T {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        (*self).pipe(pipeline)
    }
}

impl<T: Pipe> Pipe for [T] {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        for pipe in self {
            pipe.pipe(pipeline)
        }
    }
}

impl<T: Pipe> Pipe for Option<T> {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        if let Some(pipe) = self {
            pipe.pipe(pipeline)
        }
    }
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

    pub(crate) fn filled<'b, D>(self, draws: D) -> Pipeline<'b>
    where
        D: IntoIterator<Item = &'b dyn Pipe>,
    {
        let mut pipeline = self.ensure_empty();
        for draw in draws {
            draw.pipe(&mut pipeline)
        }
        pipeline
    }

    pub(crate) fn cleared<'b>(mut self) -> Pipeline<'b> {
        self.solid.clear();
        self.skin.clear();
        self.color.clear();
        self.interface.clear();
        self.ensure_empty()
    }

    fn ensure_empty<'b>(self) -> Pipeline<'b> {
        fn safe_capacity<'a, 'b, S>(mut v: Vec<&'a dyn Draw<S>>) -> Vec<&'b dyn Draw<S>> {
            unsafe {
                assert!(v.is_empty());
                let ptr = v.as_mut_ptr() as *mut *const dyn Draw<S>;
                let cap = v.capacity();
                std::mem::forget(v);
                Vec::from_raw_parts(ptr as *mut &dyn Draw<S>, 0, cap)
            }
        }

        Pipeline {
            solid: safe_capacity(self.solid),
            skin: safe_capacity(self.skin),
            color: safe_capacity(self.color),
            interface: safe_capacity(self.interface),
        }
    }
}
