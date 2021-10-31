use crate::{shader::def::*, vertex::Vertex};
use shr::cgm::*;

def_shader! {
    type: vertex,
    struct: Vertex,
    where: {
        model: Mat4,
        view: Mat4,
        proj: Mat4,
    },
    fn: () -> (fs_st: Vec2),
    impl: {
        void main() {
            fs_st = st;
            gl_Position = proj * view * model * vec4(co, 1.0);
        }
    }
}
