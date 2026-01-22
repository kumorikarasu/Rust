
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

use super::traits::IndexType;

#[derive(Serialize, Deserialize, Debug)]
pub struct DbIndex {
    pub index: &'static str,
    tree: BTreeMap<IndexType, Vec<u64>>
}

impl DbIndex
{
    pub fn get_index(&self) -> &'static str {
        self.index
    }

    pub fn new(index: &'static str) -> DbIndex {
        DbIndex {
            index,
            tree: BTreeMap::new()
        }
    }

    pub fn insert(&mut self, key: IndexType, value: u64) {
        self.tree.insert(key, vec![value]);
    }

    pub fn append(&mut self, key: IndexType, value: u64) {
        match self.tree.get_mut(&key) {
            Some(vec) => {
                vec.push(value);
            },
            None => {
                self.tree.insert(key, vec![value]);
            }
        }
    }

    pub fn find(&self, key: &IndexType) -> Vec<u64> {
        match self.tree.get(key).cloned() {
            Some(vec) => vec,
            None => Vec::new()
        }
    }
}
