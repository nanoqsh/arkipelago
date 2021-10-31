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
            frag = texture(t0, fs_st);
        }
    }
}
