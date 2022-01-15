use std::{
    collections::{hash_map::Entry, HashMap},
    ops,
    rc::Rc,
};

pub(crate) trait Load {
    type Asset;
    type Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Self::Error>;
}

pub(crate) struct Cached<L: Load> {
    map: HashMap<String, Rc<L::Asset>>,
    load: L,
}

impl<L: Load> Cached<L> {
    pub fn new(load: L) -> Self {
        Self {
            load,
            map: HashMap::default(),
        }
    }
}

impl<L: Load> ops::Deref for Cached<L> {
    type Target = L;

    fn deref(&self) -> &Self::Target {
        &self.load
    }
}

impl<L: Load> ops::DerefMut for Cached<L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.load
    }
}

impl<L: Load> Load for Cached<L> {
    type Asset = Rc<L::Asset>;
    type Error = L::Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Self::Error> {
        match self.map.entry(name.into()) {
            Entry::Occupied(en) => Ok(Rc::clone(en.get())),
            Entry::Vacant(en) => {
                let asset = Rc::new(self.load.load(name)?);
                en.insert(Rc::clone(&asset));
                Ok(asset)
            }
        }
    }
}

type Event<'a, A> = Box<dyn FnMut(&str, &A) + 'a>;

pub(crate) struct EventLoad<'a, L: Load> {
    event: Option<Event<'a, L::Asset>>,
    load: L,
}

impl<'a, L: Load> EventLoad<'a, L> {
    pub fn new(load: L) -> Self {
        Self { event: None, load }
    }

    pub fn set_event(&mut self, event: Event<'a, L::Asset>) {
        self.event = Some(event)
    }
}

impl<L: Load> ops::Deref for EventLoad<'_, L> {
    type Target = L;

    fn deref(&self) -> &Self::Target {
        &self.load
    }
}

impl<L: Load> ops::DerefMut for EventLoad<'_, L> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.load
    }
}

impl<L: Load> Load for EventLoad<'_, L> {
    type Asset = L::Asset;
    type Error = L::Error;

    fn load(&mut self, name: &str) -> Result<Self::Asset, Self::Error> {
        let asset = self.load.load(name)?;
        if let Some(event) = self.event.as_mut() {
            event(name, &asset)
        }
        Ok(asset)
    }
}
