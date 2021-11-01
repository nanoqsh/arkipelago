mod first_person;
mod rotation;
mod third_person;
mod this;

pub(crate) use self::{first_person::FpCamera, third_person::TpCamera, this::Camera};
