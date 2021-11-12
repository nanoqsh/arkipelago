mod builder;
mod overlay;
pub(crate) mod polygon;
mod shape;
pub(crate) mod variant;
mod vec_map;

pub(crate) use self::{
    builder::Builder,
    overlay::{Connections, Overlay},
    shape::{Factory, Parameters},
};
