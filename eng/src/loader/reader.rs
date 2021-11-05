use crate::loader::re::*;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    path::PathBuf,
    rc::Rc,
};

type Event<'a, A> = Box<dyn FnMut(&str, Rc<A>) + 'a>;

struct Cached<'a, A> {
    loaded: HashMap<String, Rc<A>>,
    on_load: Option<Event<'a, A>>,
}

impl<'a, A> Cached<'a, A> {
    fn with_capacity(cap: usize) -> Self {
        Self {
            loaded: HashMap::with_capacity(cap),
            on_load: None,
        }
    }

    fn load<S, F, E>(&mut self, name: S, fetch: F) -> Result<Rc<A>, E>
    where
        S: Into<String>,
        F: FnOnce(&str) -> Result<A, E>,
    {
        match self.loaded.entry(name.into()) {
            Entry::Occupied(en) => Ok(Rc::clone(en.get())),
            Entry::Vacant(en) => {
                let key = en.key();
                let asset = fetch(key)?;
                let rc = Rc::new(asset);

                if let Some(event) = &mut self.on_load {
                    event(key, Rc::clone(&rc));
                }

                Ok(Rc::clone(en.insert(rc)))
            }
        }
    }
}

pub(crate) struct Reader<'a, A, B = ()> {
    cached: Cached<'a, A>,
    buf: B,
    path: Rc<RefCell<PathBuf>>,
}

impl<'a, A, B> Reader<'a, A, B> {
    pub fn with_capacity(buf: B, path: Rc<RefCell<PathBuf>>, cap: usize) -> Self {
        Self {
            cached: Cached::with_capacity(cap),
            buf,
            path,
        }
    }

    pub fn on_load(&mut self, event: Event<'a, A>) {
        self.cached.on_load = Some(event)
    }

    pub fn take(&mut self) -> HashMap<String, Rc<A>> {
        std::mem::take(&mut self.cached.loaded)
    }

    fn read<'l, L>(
        cached: &mut Cached<A>,
        name: &str,
        load: L,
        format: L::Format,
        path: &RefCell<PathBuf>,
    ) -> Result<Rc<A>, Error>
    where
        L: Load<'l, Asset = A>,
    {
        cached.load(name, |name| {
            let raw = {
                let mut path = path.borrow_mut();
                path.clear();
                path.push(ASSETS_PATH);
                path.push(L::PATH);
                path.push(name);
                path.set_extension(L::Format::EXT);

                println!("[ DEBUG ] Read: {:?}", path);

                format.read(&path)?
            };

            load.load(raw)
        })
    }
}

impl<A> Reader<'_, A> {
    pub fn read_png<'a, L>(&mut self, name: &str, load: L) -> Result<Rc<A>, Error>
    where
        L: Load<'a, Format = Png, Asset = A>,
    {
        Self::read(&mut self.cached, name, load, Png, &self.path)
    }
}

impl<A> Reader<'_, A, String> {
    pub fn read_json<'a, 'b, L, T>(&'b mut self, name: &str, load: L) -> Result<Rc<A>, Error>
    where
        L: Load<'a, Format = Json<'b, T>, Asset = A>,
        T: Deserialize<'b>,
    {
        Self::read(
            &mut self.cached,
            name,
            load,
            Json::new(&mut self.buf),
            &self.path,
        )
    }
}
