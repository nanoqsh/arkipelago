use crate::land::polygon::Polygon;

pub(crate) enum Overlay {
    None,
    Empty,
    Polygon(Polygon),
}
