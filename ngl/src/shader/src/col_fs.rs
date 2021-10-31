use crate::shader::def::*;
use shr::cgm::*;

def_shader! {
    type: fragment,
    fn: (fs_cl: Vec3) -> (frag: Vec4),
    impl: {
        void main() {
            frag = vec4(fs_cl, 1.0);
        }
    }
}
