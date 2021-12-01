use crate::{shader::def::*, uniform::Sampler2d};
use shr::cgm::*;

def_shader! {
    type: fragment,
    where: {
        t0: Sampler2d,
        vignette_cl: Vec3,
    },
    fn: (fs_st: Vec2) -> (frag: Vec4),
    impl: {
        void main() {
            vec2 screen = fs_st * 2.0 - 1.0;
            vec3 vignette = 1.0 - length(screen) * (1.0 - vignette_cl);
            frag = texture(t0, fs_st) * vec4(vignette, 1.0);
        }
    }
}
