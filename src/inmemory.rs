use std::fmt::Debug;
use std::sync::Mutex;
use std::sync::Arc;
use crate::database_traits::Entity;
use std::collections::HashMap;
use crate::database_traits::Database;

pub struct InMemory<T> {
    ids: Arc<Mutex<i32>>,
    entities: Arc<Mutex<HashMap<i32, T>>>
}

impl<T> InMemory<T> {
    pub fn new() -> Box<InMemory<T>> {
        Box::new(
            InMemory {
                ids: Arc::new(Mutex::new(0)),
                entities: Arc::new(Mutex::new(HashMap::new()))
            }
        )
    }
}

impl<T: Entity + Clone + Debug> Database<T> for InMemory<T> {
    fn insert(&self, mut entity: T) -> Option<T> {

        *(self.ids).lock().unwrap() += 1;
        let id = self.ids.lock().unwrap().clone();

        entity.set_id(id);
        let res = entity.clone();

        match self.entities.lock().unwrap().insert(id, entity) {
            Some(_) => panic!("Entity already exists at that id"),
            None => Some(res)
        }
    }

    fn read(&self, id: i32) -> Option<T> {
        let lock = self.entities.lock().unwrap();
        lock.get(&id).cloned()
    }

    fn update(&self, entity: T) -> Option<T> {
        self.entities.lock().unwrap().insert(entity.get_id(), entity)
    }

    fn delete(&self, id: i32) -> Option<T> {
        self.entities.lock().unwrap().remove(&id)
    }
}


