use ngl::{
    pass::{Color, Pass, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;

pub(crate) struct Path(pub Vec<Vec3>);

impl Draw<Color> for Path {
    fn draw<'a>(&self, pass: Pass<'a, Color>)
    where
        Color: Stage<'a>,
    {
        pass.set_model(&Mat4::identity());
        for (i, edge) in self.0.windows(2).enumerate() {
            let (a, b) = match *edge {
                [a, b] => (a, b),
                _ => unreachable!(),
            };

            let cl = {
                let len = self.0.len();
                let f = (len - i) as f32 / len as f32;
                Vec3::new(0.9, f, 1.0 - f)
            };

            pass.draw_line(a, b, cl);
        }
    }
}

impl Pipe for Path {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        pipeline.push_color(self)
    }
}
