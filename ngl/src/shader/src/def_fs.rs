use crate::{shader::def::*, uniform::Sampler2d};
use shr::cgm::*;

def_shader! {
    type: fragment,
    const: {
        NEAR: 0.1f32,
        FAR: 100.0f32,
    },
    where: {
        t0: Sampler2d,
        fog_cl: Vec3,
        fog_near: f32,
        fog_far: f32,
    },
    fn: (fs_co: Vec3, fs_st: Vec2) -> (frag: Vec4),
    impl: {
        const float NEAR = $NEAR;
        const float FAR = $FAR;

        void main() {
            vec4 cl = texture(t0, fs_st);
            if (cl.a < 0.9) {
                discard;
            }

            float n = gl_FragCoord.z * 2.0 - 1.0;
            float dist = FAR - NEAR;
            float d = (2.0 * NEAR * FAR) / (FAR + NEAR - n * dist);
            
            float fog_factor = ((NEAR + fog_far * dist) - length(fs_co)) / ((fog_far - fog_near) * dist);
            fog_factor = clamp(fog_factor, 0.0, 1.0);

            frag = mix(vec4(fog_cl, 1.0), cl, fog_factor);
        }
    }
}
