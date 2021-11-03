use crate::loader::{cached::Cached, re::*};
use std::{path::PathBuf, rc::Rc};

pub(crate) struct Reader<'a, A, B = ()> {
    cached: Cached<'a, A>,
    buf: B,
}

impl<'a, A, B> Reader<'a, A, B> {
    pub fn with_capacity(buf: B, cap: usize) -> Self {
        Self {
            cached: Cached::with_capacity(cap),
            buf,
        }
    }

    pub fn cached(&mut self) -> &mut Cached<'a, A> {
        &mut self.cached
    }

    fn read<'l, L>(
        cached: &mut Cached<A>,
        name: &str,
        load: L,
        format: L::Format,
    ) -> Result<Rc<A>, Error>
    where
        L: Load<'l, Asset = A>,
    {
        cached.load(name, |name| {
            let mut path = PathBuf::new();
            let path = &mut path;

            path.clear();
            path.push(ASSETS_PATH);
            path.push(L::PATH);
            path.push(name);
            path.set_extension(L::Format::EXT);

            println!("[ DEBUG ] Read: {:?}", path);

            let raw = format.read(path)?;
            load.load(raw)
        })
    }
}

impl<A> Reader<'_, A> {
    pub fn read_png<'a, L>(&mut self, name: &str, load: L) -> Result<Rc<A>, Error>
    where
        L: Load<'a, Format = Png, Asset = A>,
    {
        Self::read(&mut self.cached, name, load, Png)
    }
}

impl<A> Reader<'_, A, String> {
    pub fn read_json<'a, 'b, L, T>(&'b mut self, name: &str, load: L) -> Result<Rc<A>, Error>
    where
        L: Load<'a, Format = Json<'b, T>, Asset = A>,
        T: Deserialize<'b>,
    {
        Self::read(&mut self.cached, name, load, Json::new(&mut self.buf))
    }
}
