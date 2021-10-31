use crate::{shader::def::*, vertex::ColorVertex};
use shr::cgm::*;

def_shader! {
    type: vertex,
    struct: ColorVertex,
    where: {
        model: Mat4,
        view: Mat4,
        proj: Mat4,
    },
    fn: () -> (fs_cl: Vec3),
    impl: {
        void main() {
            fs_cl = cl;
            gl_Position = proj * view * model * vec4(co, 1.0);
        }
    }
}
