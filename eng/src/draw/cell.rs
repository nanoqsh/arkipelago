use ngl::{
    pass::{Color, Pass, Stage},
    Draw, Pipe, Pipeline,
};
use shr::cgm::*;

pub(crate) struct Cell(pub Vec3);

impl Draw<Color> for Cell {
    fn draw<'a>(&self, pass: Pass<'a, Color>)
    where
        Color: Stage<'a>,
    {
        let verts = [
            (-0.5, -0.5),
            (-0.5, 0.5),
            (0.5, 0.5),
            (0.5, -0.5),
            (-0.5, -0.5),
        ];

        pass.set_model(&Mat4::identity());
        for edge in verts.windows(2) {
            let (a, b) = match *edge {
                [(x0, z0), (x1, z1)] => (Vec3::new(x0, 0., z0), Vec3::new(x1, 0., z1)),
                _ => unreachable!(),
            };

            pass.draw_line(self.0 + a, self.0 + b, Vec3::new(0.5, 1., 0.1));
        }
    }
}

impl Pipe for Cell {
    fn pipe<'a>(&'a self, pipeline: &mut Pipeline<'a>) {
        pipeline.push_color(self)
    }
}
