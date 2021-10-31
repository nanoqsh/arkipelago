use glow::Context;

pub(crate) trait ClearContext {
    fn clear(&mut self, ctx: &Context);
}
