use std::{
    collections::{hash_map::Entry, HashMap},
    rc::Rc,
};

type Event<A> = Box<dyn FnMut(&str, Rc<A>)>;

pub(crate) struct Cached<A> {
    loaded: HashMap<String, Rc<A>>,
    on_load: Vec<Event<A>>,
}

impl<A> Cached<A> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            loaded: HashMap::with_capacity(cap),
            on_load: Vec::default(),
        }
    }

    pub fn load<S, F, E>(&mut self, name: S, fetch: F) -> Result<Rc<A>, E>
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

                for event in &mut self.on_load {
                    event(key, Rc::clone(&rc));
                }

                en.insert(Rc::clone(&rc));
                Ok(rc)
            }
        }
    }

    pub fn on_load(&mut self, event: Event<A>) {
        self.on_load.push(event)
    }
}
