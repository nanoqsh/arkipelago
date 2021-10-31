use crate::{
    attribute::Attribute,
    program::{checker::ProgramChecker, Program},
};
use glow::{NativeProgram, NativeUniformLocation};
use std::{collections::HashMap, marker::PhantomData};

pub(crate) struct UniformInfo {
    nat: NativeUniformLocation,
    typ: u32,
    len: u32,
}

impl UniformInfo {
    pub fn new(nat: NativeUniformLocation, typ: u32, len: u32) -> Self {
        Self { nat, typ, len }
    }
}

pub(crate) struct Location<T>
where
    T: ?Sized,
{
    nat: NativeUniformLocation,
    checker: ProgramChecker,
    typ: PhantomData<T>,
}

impl<T> Clone for Location<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Location<T> {}

impl<T> Location<T>
where
    T: ?Sized,
{
    fn new(nat: NativeUniformLocation, program: NativeProgram) -> Self {
        Self {
            nat,
            checker: ProgramChecker::new(program),
            typ: PhantomData,
        }
    }

    pub fn nat(&self) -> NativeUniformLocation {
        self.nat
    }

    pub fn check(&self, other: NativeProgram) -> bool {
        self.checker.check(other)
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn as_ref<U>(self) -> Location<U>
    where
        T: AsRef<U>,
        U: ?Sized,
    {
        Location {
            nat: self.nat,
            checker: self.checker,
            typ: PhantomData,
        }
    }
}

pub(crate) struct Locations<'a> {
    uniforms: HashMap<String, UniformInfo>,
    program: &'a Program,
}

impl<'a> Locations<'a> {
    pub fn with_capacity(cap: usize, program: &'a Program) -> Self {
        Self {
            uniforms: HashMap::with_capacity(cap),
            program,
        }
    }

    pub fn insert(&mut self, name: String, uni: UniformInfo) {
        let old = self.uniforms.insert(name, uni);
        assert!(old.is_none())
    }

    pub fn get<T>(&self, name: &str) -> Option<Location<T>>
    where
        T: Attribute,
    {
        let uni = self.uniforms.get(name)?;
        (T::SYMBOLIC == uni.typ && T::ARRAY_LEN == uni.len)
            .then(|| Location::new(uni.nat, self.program.nat()))
    }
}
