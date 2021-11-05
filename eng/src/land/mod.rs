mod overlay;
pub(crate) mod polygon;
mod shape;
mod shape_factory;
pub(crate) mod variant;

pub(crate) use self::{
    overlay::Overlay,
    shape_factory::{Factory, Parameters},
};
