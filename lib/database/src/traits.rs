use super::index::DbIndex;

pub trait Entity {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

pub trait Database<T: Sized + Entity> {
    fn insert_one(&self, entity: T) -> Option<T>;
    fn read(&self, id: u64) -> Option<T>;
    fn read_all(&self) -> Vec<T>;
    fn update(&self, id: u64, entity: T) -> Option<T>;
    fn delete(&self, id: u64) -> Option<T>;
}

pub trait Indexable<T> { 
    fn add_index(&self, Entity: &T);
    fn search(&self, index: &str, value: &str) -> u64;
}

pub trait Index {
    fn get_indexes(&self) -> Vec<(&str, IndexType)>;
}

pub trait IndexableDatabase<T: Sized + Entity>: Database<T> + Indexable<T> {
    fn insert(&self, entity: T) -> Option<T>;
}

pub enum IndexType {
    String (String),
    u64 (u64),
    u32 (u32),
    u16 (u16),
    i64 (i64),
    i32 (i32),
    i16 (i16),
}
