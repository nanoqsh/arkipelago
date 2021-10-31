use glow::NativeProgram;

#[cfg(debug_assertions)]
#[derive(Copy, Clone)]
pub(crate) struct ProgramChecker(NativeProgram);

#[cfg(not(debug_assertions))]
#[derive(Copy, Clone)]
pub(crate) struct ProgramChecker();

impl ProgramChecker {
    #[cfg(debug_assertions)]
    pub fn new(program: NativeProgram) -> Self {
        Self(program)
    }

    #[cfg(not(debug_assertions))]
    pub fn new(_: NativeProgram) -> Self {
        Self()
    }

    #[cfg(debug_assertions)]
    pub fn check(&self, program: NativeProgram) -> bool {
        self.0 == program
    }

    #[cfg(not(debug_assertions))]
    pub fn check(&self, program: NativeProgram) -> bool {
        unreachable!()
    }
}
