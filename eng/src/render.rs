use crate::{IndexedMesh, Texture, Vert};
use image::{DynamicImage, GenericImageView};
use ngl::{Parameters, Pipe};
use shr::cgm::*;

type Ren = ngl::Render;

pub struct Render {
    ren: Ren,
    view: Option<Mat4>,
    proj: Option<Mat4>,
}

impl Render {
    /// # Safety
    ///   - Pass correct `load` function.
    ///   - The context must be current and don't changes.
    pub unsafe fn new<F>(load: F) -> Self
    where
        F: FnMut(&str) -> *const (),
    {
        use std::cell::Cell;

        thread_local! {
            static INIT: Cell<bool> = Cell::new(false);
        }

        INIT.with(|initialized| {
            if initialized.get() {
                panic!("render has already been created once")
            }

            initialized.set(true);
        });

        Self {
            ren: Ren::new(load),
            view: None,
            proj: None,
        }
    }

    pub fn make_texture(&self, image: &DynamicImage) -> Texture {
        use ngl::texture::*;

        let (data, format) = match image {
            DynamicImage::ImageLuma8(data) => (data.as_raw(), Format::R),
            DynamicImage::ImageLumaA8(data) => (data.as_raw(), Format::Rg),
            DynamicImage::ImageRgb8(data) => (data.as_raw(), Format::Rgb),
            DynamicImage::ImageRgba8(data) => (data.as_raw(), Format::Rgba),
            _ => panic!("unsupported image format"),
        };

        let size = image.dimensions();
        let params = Parameters {
            format,
            ..Parameters::default()
        };

        self.ren.make_texture(data, size.into(), params)
    }

    pub fn make_mesh(&self, verts: &[Vert], indxs: &[u32]) -> IndexedMesh {
        self.ren.make_indexed_mesh(verts, indxs)
    }

    pub fn resize(&mut self, size: (u32, u32)) {
        self.ren.resize(size.into())
    }

    pub fn set_view(&mut self, view: Mat4) {
        self.view = Some(view)
    }

    pub fn set_proj(&mut self, proj: Mat4) {
        self.proj = Some(proj)
    }

    pub fn draw<'a, D>(&mut self, draws: D)
    where
        D: IntoIterator<Item = &'a dyn Pipe>,
    {
        self.ren.draw(
            draws,
            Parameters {
                cl: Vec3::new(0., 0., 0.),
                view: self.view.take().as_ref(),
                proj: self.proj.take().as_ref(),
            },
        )
    }
}
