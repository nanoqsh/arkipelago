use crate::{
    program::{location::UniformInfo, Location, Locations},
    shader::Shader,
    uniform::Uniform,
};
use glow::{ActiveUniform, Context, HasContext, NativeProgram};
use std::{cell::Cell, rc::Rc};

pub(crate) struct Program {
    nat: NativeProgram,
    ctx: Rc<Context>,
}

impl Program {
    thread_local! {
        static USED: Cell<Option<NativeProgram>> = Cell::new(None);
    }

    pub fn new<'a, S>(ctx: Rc<Context>, shaders: S) -> Self
    where
        S: IntoIterator<Item = Shader<'a>>,
    {
        unsafe {
            let nat = ctx.create_program().expect("create program");

            let mut buf = String::with_capacity(64);
            let shaders: Vec<_> = shaders
                .into_iter()
                .map(|shader| {
                    buf.clear();
                    shader.write_glsl(&mut buf);
                    let shader = ctx.create_shader(shader.gl()).expect("create shader");

                    ctx.shader_source(shader, &buf);
                    ctx.compile_shader(shader);

                    if cfg!(debug_assertions) {
                        println!("[ DEBUG ] Shader:\n{buf}");
                    }

                    if !ctx.get_shader_compile_status(shader) {
                        let log = ctx.get_shader_info_log(shader);
                        panic!("{log}");
                    }

                    ctx.attach_shader(nat, shader);
                    shader
                })
                .collect();

            ctx.link_program(nat);
            if !ctx.get_program_link_status(nat) {
                let log = ctx.get_program_info_log(nat);
                panic!("{log}");
            }

            for shader in shaders {
                ctx.detach_shader(nat, shader);
                ctx.delete_shader(shader);
            }

            Self { nat, ctx }
        }
    }

    pub fn nat(&self) -> NativeProgram {
        self.nat
    }

    pub fn use_program(&self) {
        let prog = Some(self.nat);
        unsafe { self.ctx.use_program(prog) }

        debug_assert!(Self::USED.with(|used| {
            used.set(prog);
            true
        }));
    }

    pub fn locations(&self) -> Locations {
        unsafe {
            let n = self.ctx.get_active_uniforms(self.nat);
            let mut locs = Locations::with_capacity(n as _, self);

            for i in 0..n {
                let mut uni = self.ctx.get_active_uniform(self.nat, i).unwrap();
                let nat = self.ctx.get_uniform_location(self.nat, &uni.name).unwrap();

                if let Some(idx) = uni.name.find('[') {
                    uni.name.replace_range(idx.., "");
                }

                if cfg!(debug_assertions) {
                    let ActiveUniform { name, size, .. } = &uni;
                    println!("[ DEBUG ] Var: {name} {size}");
                }

                locs.insert(uni.name, UniformInfo::new(nat, uni.utype, uni.size as _))
            }

            locs
        }
    }

    pub fn uniform<T>(&self, loc: Location<T>, val: &T)
    where
        T: Uniform + ?Sized,
    {
        debug_assert!(
            loc.check(self.nat),
            "this location belongs to another program"
        );

        debug_assert_eq!(
            Self::USED.with(Cell::get),
            Some(self.nat),
            "the program is not using"
        );

        val.uniform(&self.ctx, loc.nat())
    }

    pub fn uniform_slice<T, const N: usize>(&self, loc: Location<[T; N]>, slice: &[T])
    where
        T: Uniform,
    {
        assert!(slice.len() <= N);
        self.uniform(loc.as_ref(), slice);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { self.ctx.delete_program(self.nat) }
    }
}
