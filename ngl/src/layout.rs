use crate::attribute::Components;
pub(crate) use ngl_derive::Layout;

pub(crate) struct Field {
    pub name: &'static str,
    pub declare: fn(&mut String),
    pub offset: u32,
    pub components: Components,
    pub element_type: u32,
}

pub(crate) trait Layout {
    fn layout(fields: &mut Vec<Field>);
}

impl Layout for () {
    fn layout(_: &mut Vec<Field>) {}
}
