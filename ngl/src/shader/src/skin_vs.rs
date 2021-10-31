use crate::{
    shader::{def::*, BONES_MAX_LEN},
    vertex::SkinVertex,
};
use shr::cgm::*;

def_shader! {
    type: vertex,
    struct: SkinVertex,
    const: {
        BONES_MAX_LEN: BONES_MAX_LEN as u32,
    },
    where: {
        model: Mat4,
        view: Mat4,
        proj: Mat4,
        bones: [Mat4; BONES_MAX_LEN],
    },
    fn: () -> (fs_st: Vec2),
    impl: {
        const uint BONES_MAX_LEN = $BONES_MAX_LEN;

        void main() {
            mat4 bone =
                  bones[bs.x] * ws.x
                + bones[bs.y] * ws.y
                + bones[bs.z] * ws.z;

            fs_st = st;
            gl_Position = proj * view * model * bone * vec4(co, 1.0);
        }
    }
}
