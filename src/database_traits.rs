pub trait Entity {
    fn get_id(&self) -> i32;
    fn set_id(&mut self, id: i32);
}

pub trait Database<T: Sized + Entity> {
    fn insert(&self, entity: T) -> Option<T>;
    fn read(&self, id: i32) -> Option<T>;
    fn update(&self, entity: T) -> Option<T>;
    fn delete(&self, id: i32) -> Option<T>;
}
