use crate::{shader::def::*, uniform::Sampler2d};
use shr::cgm::*;

def_shader! {
    type: fragment,
    where: {
        t0: Sampler2d,
    },
    fn: (fs_st: Vec2) -> (frag: Vec4),
    impl: {
        void main() {
            vec4 col = texture(t0, fs_st);
            if (col.a < 0.9) {
                discard;
            }

            frag = col;
        }
    }
}
