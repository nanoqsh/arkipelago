use crate::{line::Line, mesh::Indexed, shaders::*, texture::Texture, vertex::Vertex, Fog};
use shr::cgm::*;
use std::ops;

pub trait Stage<'a> {
    type Inner;
}

pub struct Pass<'a, S: Stage<'a>>(pub(crate) S::Inner);

impl Pass<'_, Solid> {
    pub fn draw_indexed_mesh(&self, mesh: &Indexed<Vertex>) {
        mesh.bind();
        mesh.draw();
    }

    pub fn set_texture(&self, tex: &Texture) {
        tex.bind(Shaders::T0)
    }

    pub fn set_model(&self, model: &Mat4) {
        self.shader.set_model(model);
    }
}

impl Pass<'_, Skin> {
    pub fn set_texture(&self, tex: &Texture) {
        tex.bind(Shaders::T0)
    }

    pub fn set_model(&self, model: &Mat4) {
        self.shader.set_model(model);
    }
}

impl Pass<'_, Color> {
    pub fn draw_line(&self, a: Vec3, b: Vec3, cl: Vec3) {
        self.line.draw(a, b, cl);
    }

    pub fn set_model(&self, model: &Mat4) {
        self.shader.set_model(model);
    }
}

impl Pass<'_, Interface> {
    pub fn set_texture(&self, tex: &Texture) {
        tex.bind(Shaders::T0)
    }
}

impl<'a, S: Stage<'a>> ops::Deref for Pass<'a, S> {
    type Target = S::Inner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Solid;

#[derive(Copy, Clone)]
pub struct SolidInner<'a> {
    shader: &'a SolidProgram,
}

impl<'a> SolidInner<'a> {
    pub(crate) fn new(shader: &'a SolidProgram) -> Self {
        shader.use_program();
        Self { shader }
    }

    pub(crate) fn set_fog(&self, fog: Fog) {
        self.shader.set_fog_cl(&fog.cl);
        self.shader.set_fog_near(&fog.near);
        self.shader.set_fog_far(&fog.far);
    }

    pub(crate) fn set_view(&self, view: &Mat4) {
        self.shader.set_view(view)
    }

    pub(crate) fn set_proj(&self, proj: &Mat4) {
        self.shader.set_proj(proj)
    }
}

impl<'a> Stage<'a> for Solid {
    type Inner = SolidInner<'a>;
}

pub struct Skin;

#[derive(Copy, Clone)]
pub struct SkinInner<'a> {
    shader: &'a SkinProgram,
}

impl<'a> SkinInner<'a> {
    pub(crate) fn new(shader: &'a SkinProgram) -> Self {
        shader.use_program();
        Self { shader }
    }

    pub(crate) fn set_fog(&self, fog: Fog) {
        self.shader.set_fog_cl(&fog.cl);
        self.shader.set_fog_near(&fog.near);
        self.shader.set_fog_far(&fog.far);
    }

    pub(crate) fn set_view(&self, view: &Mat4) {
        self.shader.set_view(view)
    }

    pub(crate) fn set_proj(&self, proj: &Mat4) {
        self.shader.set_proj(proj)
    }
}

impl<'a> Stage<'a> for Skin {
    type Inner = SkinInner<'a>;
}

pub struct Color;

#[derive(Copy, Clone)]
pub struct ColorInner<'a> {
    shader: &'a ColorProgram,
    line: &'a Line,
}

impl<'a> ColorInner<'a> {
    pub(crate) fn new(shader: &'a ColorProgram, line: &'a Line) -> Self {
        shader.use_program();
        Self { shader, line }
    }

    pub(crate) fn set_view(&self, view: &Mat4) {
        self.shader.set_view(view)
    }

    pub(crate) fn set_proj(&self, proj: &Mat4) {
        self.shader.set_proj(proj)
    }
}

impl<'a> Stage<'a> for Color {
    type Inner = ColorInner<'a>;
}

pub struct Interface;

#[derive(Copy, Clone)]
pub struct InterfaceInner(pub(crate) ());

impl Stage<'_> for Interface {
    type Inner = InterfaceInner;
}
