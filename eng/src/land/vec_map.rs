pub(crate) struct Map<T> {
    map: Vec<Option<T>>,
}

impl<T: Copy> Map<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: vec![None; cap],
        }
    }

    pub fn insert(&mut self, key: u32, val: T) -> bool {
        unsafe {
            let key = key as usize;
            if key >= self.map.len() {
                let additional = key - self.map.len() + 1;
                self.map.extend(std::iter::repeat(None).take(additional));
            }

            debug_assert!(key < self.map.len());
            let cell = self.map.get_unchecked_mut(key);
            match cell {
                None => {
                    *cell = Some(val);
                    true
                }
                _ => false,
            }
        }
    }

    pub unsafe fn get_unchecked(&self, key: u32) -> T {
        let key = key as usize;
        debug_assert!(key < self.map.len());
        let val = self.map.get_unchecked(key);
        debug_assert!(val.is_some());
        val.unwrap_or_else(|| std::hint::unreachable_unchecked())
    }

    #[cfg(test)]
    pub fn get(&self, key: u32) -> T {
        self.map[key as usize].unwrap()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut map = Map::with_capacity(10);
        assert!(map.insert(5, 7));
        assert!(!map.insert(5, 8));
        assert_eq!(map.get(5), 7);

        assert!(map.insert(100, 7));
        assert_eq!(map.get(100), 7);
    }
}
