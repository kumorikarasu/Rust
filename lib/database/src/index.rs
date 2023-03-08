
use std::collections::BTreeMap;

pub struct DbIndex<T> {
    pub index: &'static str,
    tree: BTreeMap<T, u64>
}

impl<T> DbIndex<T> 
where T: Ord + Clone + std::fmt::Debug
{
    pub fn get_index(&self) -> &'static str {
        self.index
    }

    pub fn new(index: &'static str) -> DbIndex<T> {
        DbIndex {
            index,
            tree: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, key: T, value: u64) {
        self.tree.insert(key, value);
    }

    pub fn get(&self, key: &T) -> Option<&u64> {
        self.tree.get(key)
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut u64> {
        self.tree.get_mut(key)
    }

    pub fn remove(&mut self, key: &T) -> Option<u64> {
        self.tree.remove(key)
    }

    pub fn contains_key(&self, key: &T) -> bool {
        self.tree.contains_key(key)
    }
}
