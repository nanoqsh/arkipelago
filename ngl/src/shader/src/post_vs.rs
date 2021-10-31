use crate::{shader::def::*, vertex::PostVertex};
use shr::cgm::*;

def_shader! {
    type: vertex,
    struct: PostVertex,
    fn: () -> (fs_st: Vec2),
    impl: {
        void main() {
            fs_st = st;
            gl_Position = vec4(co, 0.0, 1.0);
        }
    }
}
