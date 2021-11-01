use crate::{camera::TpCamera, render::Data, Render};
use ngl::{
    pass::{Pass, Solid, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;

impl Draw<Solid> for Data {
    fn draw<'a>(&self, pass: Pass<'a, Solid>)
    where
        Solid: Stage<'a>,
    {
        pass.set_model(&Mat4::identity());
        pass.set_texture(&self.tex);
        pass.draw_indexed_mesh(&self.mesh);
    }
}

impl Pipe for Data {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        pipeline.push_solid(self);
    }
}

#[derive(Debug)]
pub enum Control {
    Look(f32, f32),
    Scroll(f32, f32),
    Forward,
    Back,
    Left,
    Right,
}

pub struct Game {
    data: Data,
    cam: TpCamera,
}

impl Game {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Data) -> Self {
        Self {
            data,
            cam: TpCamera::new(2., Pnt3::origin()),
        }
    }

    pub fn draw(&mut self, ren: &mut Render, delta: f32) {
        ren.set_proj(self.cam.proj(1.));
        ren.set_view(self.cam.view());
        ren.draw([&self.data as &dyn Pipe])
    }

    pub fn input(&mut self, control: Control) {
        const SENSITIVITY: f32 = 0.01;

        match control {
            Control::Look(x, y) => self.cam.rotate(Vec2::new(x, y) * SENSITIVITY),
            Control::Scroll(_, y) => self.cam.move_to(y),
            Control::Forward => {}
            Control::Back => {}
            Control::Left => {}
            Control::Right => {}
        }
    }
}
