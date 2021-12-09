use glow::{Context, HasContext};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum GlError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    ContextLost,
}

impl GlError {
    fn from_error(code: u32) -> Self {
        match code {
            glow::INVALID_ENUM => Self::InvalidEnum,
            glow::INVALID_VALUE => Self::InvalidValue,
            glow::INVALID_OPERATION => Self::InvalidOperation,
            glow::STACK_OVERFLOW => Self::StackOverflow,
            glow::STACK_UNDERFLOW => Self::StackUnderflow,
            glow::OUT_OF_MEMORY => Self::OutOfMemory,
            glow::INVALID_FRAMEBUFFER_OPERATION => Self::InvalidFramebufferOperation,
            glow::CONTEXT_LOST => Self::ContextLost,
            _ => panic!("undefined error code"),
        }
    }
}

pub(crate) struct Debugger {
    ctx: Rc<Context>,
}

impl Debugger {
    pub fn new(ctx: Rc<Context>) -> Self {
        Self { ctx }
    }

    pub fn check_error(&self) -> Result<(), GlError> {
        match unsafe { self.ctx.get_error() } {
            glow::NO_ERROR => Ok(()),
            err => Err(GlError::from_error(err)),
        }
    }
}

macro_rules! debug_gl {
    ($this:expr) => {
        debug_assert_eq!(crate::debug::Debugger::check_error(&$this), Ok(()));
    };
}

pub(crate) use debug_gl;
