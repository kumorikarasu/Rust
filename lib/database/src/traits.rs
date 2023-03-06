pub trait Entity {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

pub trait Database<T: Sized + Entity> {
    fn insert(&self, entity: T) -> Option<T>;
    fn read(&self, id: u64) -> Option<T>;
    fn read_all(&self) -> Vec<T>;
    fn update(&self, entity: T) -> Option<T>;
    fn delete(&self, id: u64) -> Option<T>;
}
