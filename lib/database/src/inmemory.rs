use std::cmp::max;
use std::io::Seek;
use std::fmt::Debug;
use std::io::Read;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::io::Write;
use std::fs::File;
use std::sync::RwLock;
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use std::collections::HashMap;

use super::traits::Database;
use super::traits::Entity;

pub struct InMemory<T>
where T: Entity {
    ids: Arc<Mutex<u64>>,

    entities: Arc<RwLock<HashMap<u64, T>>>,

    // No point in using rwlock here due to reading still needs to deref
    file: Option<Arc<Mutex<File>>>
}

impl<T> InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Debug + Clone + Send + Sync + 'static
{
    pub fn new() -> Box<InMemory<T>> {
        Box::new(
            InMemory {
                ids: Arc::new(Mutex::new(0)),
                entities: Arc::new(RwLock::new(HashMap::new())),
                file: None
            }
        )
    }

    pub fn new_with_file(file: File) -> Box<InMemory<T>> {
        let mut db = Box::new(
            InMemory {
                ids: Arc::new(Mutex::new(0)),
                entities: Arc::new(RwLock::new(HashMap::new())),
                file: Some(Arc::new(Mutex::new(file)))
            }
        );
        db.read_from_file();
        db
    }

    pub fn write_to_file(file: Arc<Mutex<File>>, entities: Arc<RwLock<HashMap<u64, T>>>, empty: bool) {
        let mut file = match file.lock() {
            Ok(f) => f,
            Err(e) => panic!("Error locking file: {}", e)
        };
        let vec = entities.read().unwrap();
        let col = vec.iter().map(|(_, v)| v).collect::<Vec<&T>>();
        let json = serde_json::to_string(&col).unwrap();
        if empty {
            file.set_len(0).unwrap();
        }
        file.seek(std::io::SeekFrom::Start(0)).unwrap();
        file.write(json.as_bytes()).unwrap();
    }

    pub fn read_from_file(&mut self) {
        match &self.file {
            Some(file) => {
                let mut file = file.lock().unwrap();
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => (),
                    Err(_) => { println!("Error reading file"); return }
                }

                if contents.len() == 0 {
                    return;
                }

                let vec: Vec<T> = serde_json::from_str(&contents).unwrap();
                
                let mut entitymap = self.entities.write().unwrap();
                let mut id: u64 = 0;
                for entity in vec {
                    id = max(id, entity.get_id());
                    entitymap.insert(entity.get_id(), entity);
                }
                self.ids = Arc::new(Mutex::new(id));
            }
            None => panic!("No file specified")
        };
    }
}


impl<T> Database<T> for InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Clone + Debug + Send + Sync + 'static
{
    fn insert(&self, mut entity: T) -> Option<T> {

        *(self.ids).lock().unwrap() += 1;
        let id = self.ids.lock().unwrap().clone();

        entity.set_id(id);
        let centity = entity.clone();

        let result = match self.entities.write().unwrap().insert(id, entity) {
            Some(_) => panic!("Entity already exists at that id"),
            None => Some(centity)
        };

        if (self.file).is_some() {
            let pass = self.file.clone().unwrap().clone();
            let pass2 = self.entities.clone();
            thread::spawn(move || { InMemory::<T>::write_to_file(pass, pass2, false); });
        }
        result
    }

    fn read(&self, id: u64) -> Option<T> {
        let lock = self.entities.read().unwrap();
        lock.get(&id).cloned()
    }

    fn read_all(&self) -> Vec<T> {
        let lock = self.entities.read().unwrap();
        lock.values().cloned().collect()
    }

    fn update(&self, entity: T) -> Option<T> {
        let res = match entity.get_id() {
            0 => panic!("Entity does not have an id"),
            id => self.entities.write().unwrap().insert(id, entity)
        };

        if (self.file).is_some() {
            let pass = self.file.clone().unwrap().clone();
            let pass2 = self.entities.clone();
            thread::spawn(move || { InMemory::<T>::write_to_file(pass, pass2, false); });
        }

        res
    }

    fn delete(&self, id: u64) -> Option<T> {
        let res = self.entities.write().unwrap().remove(&id);

        if (self.file).is_some() {
            let pass = self.file.clone().unwrap().clone();
            let pass2 = self.entities.clone();
            thread::spawn(move || { 
                InMemory::<T>::write_to_file(pass, pass2, true); 
            });
        }

        res
    }
}
