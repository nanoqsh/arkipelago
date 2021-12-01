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
    fn: () -> (fs_co: Vec3, fs_st: Vec2),
    impl: {
        void main() {
            fs_st = st;

            vec4 res = proj * view * model * vec4(co, 1.0);
            fs_co = vec3(res);
            gl_Position = res;
        }
    }
}
