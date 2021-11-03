use crate::land::polygon::Polygon;

#[derive(Copy, Clone)]
pub(crate) enum Overlay {
    None,
    Full,
    Polygon(&'static Polygon),
}

impl Default for Overlay {
    fn default() -> Self {
        Self::None
    }
}
