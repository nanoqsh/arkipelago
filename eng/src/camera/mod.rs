#[allow(dead_code)]
mod first_person;
mod rotation;
mod third_person;
mod this;

pub(crate) use self::{third_person::TpCamera, this::Camera};
