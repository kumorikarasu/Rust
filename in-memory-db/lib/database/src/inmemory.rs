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

use super::traits::Index;
use super::traits::Timestamp;
use super::traits::Database;
use super::traits::Indexable;
use super::traits::IndexableDatabase;
use super::traits::Entity;
use super::index::DbIndex;
use super::traits::IndexType;

#[derive(Clone)]
pub struct InMemory<T>
where T: Entity {
    ids: Arc<Mutex<u64>>,

    entities: Arc<RwLock<HashMap<u64, T>>>,

    // No point in using rwlock here due to reading still needs to deref
    file: Option<Arc<Mutex<File>>>,

    indexes: Arc<RwLock<Vec<Arc<Mutex<DbIndex>>>>>
}


impl<T> InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Index + Timestamp + Debug + Clone + Send + Sync + 'static
{
    pub fn new() -> Box<InMemory<T>> {
        Box::new(
            InMemory {
                ids: Arc::new(Mutex::new(0)),
                entities: Arc::new(RwLock::new(HashMap::new())),
                file: None,
                indexes: Arc::new(RwLock::new(Vec::new()))
            }
        )
    }

    pub fn new_with_file(file: File) -> Box<InMemory<T>> {
        let mut db = Box::new(
            InMemory {
                ids: Arc::new(Mutex::new(0)),
                entities: Arc::new(RwLock::new(HashMap::new())),
                file: Some(Arc::new(Mutex::new(file))),
                indexes: Arc::new(RwLock::new(Vec::new()))
            }
        );
        db.read_from_file();
        db
    }

    // Saves the database to a file,
    // if empty is true, the file will be truncated, this is due to the fact when data is deleted
    // it rewrites the file otherwise the json will get mangled.
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

    // Reads the database from a file and regenerates indexes
    // TODO: Move indexes to a seperate file
    pub fn read_from_file(&self) {
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
                println!("Vec: {:?}", vec);
                
                let mut entitymap = self.entities.write().unwrap();
                let mut id: u64 = 0;
                for entity in vec {
                    println!("Entity: {:?}", entity);
                    id = max(id, entity.get_id());
                    self.add_index(&entity);
                    entitymap.insert(entity.get_id(), entity);
                }
                *(self.ids).lock().unwrap() = id;
            }
            None => panic!("No file specified")
        };
    }
}

impl<T> IndexableDatabase<T> for InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Timestamp + Index + Clone + Debug + Send + Sync + 'static
{
    fn insert (&self, entity: T) -> Option<T> {
        let  entity = self.insert_one(entity);
        self.add_index(entity.as_ref().unwrap());
        entity
    }

    fn update(&self, id: u64, entity: T) -> Option<T> {
        self.update_one(id, entity)
    }
}

impl<T> Indexable<T> for InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Index + Timestamp + Clone + Debug + Send + Sync + 'static
{
    // We loop through the indexes on an entity, and for each one we add its value to the index on
    // the database. If the index does not exist, we create it.
    fn add_index(&self, entity: &T)
    {
        let mut new_indexes = Vec::<DbIndex>::new();
        let id = entity.get_id();
        for index in entity.get_indexes() {
            let index_name = index.0.clone();
            let dbindexread = self.indexes.read().unwrap();

            let dbindex = dbindexread.iter().find(|x| {
                x.lock().unwrap().index == index.0
            });

            match dbindex {
                Some(dbindex) => {
                    let mut dbindexvec = dbindex.lock().unwrap();
                    dbindexvec.append(index.1, id);
                    continue;
                }
                None => {
                    // It is fine to leak the string here as it will be used for the lifetime of the program
                    let mut newdbindex = DbIndex::new(Box::leak(index_name.to_string().into_boxed_str()));
                    newdbindex.insert(index.1, id);
                    new_indexes.push(newdbindex);
                }
            }
        }

        if new_indexes.len() == 0 {
            return;
        }

        let mut indexes = self.indexes.write().unwrap();
        for index in new_indexes {
            indexes.push(Arc::new(Mutex::new(index)));
        }

    }

    // We loop through the indexes on an entity, when we find one that matches we search for the
    // value and return all keys that match
    fn search(&self, index: &str, value: IndexType) -> Vec<T> {
        for i in self.indexes.read().unwrap().iter() {
            let i = i.lock().unwrap();
            if index == i.index {
                let id = i.find(&value);
                //TODO: replace filter with getting each key directly from the hashmap
                let res = self.entities.read().unwrap().iter().filter(|(id2, _)| {
                    id.contains(id2)
                }).map(|(_, v)| v.clone()).collect::<Vec<T>>();
                return res
            }
        }

        Vec::new()
    }
}


impl<T> Database<T> for InMemory<T>
where T: Serialize + DeserializeOwned + Entity + Index + Timestamp + Clone + Debug + Send + Sync + 'static
{
    fn insert_one(&self, mut entity: T) -> Option<T> {

        *(self.ids).lock().unwrap() += 1;
        let id = self.ids.lock().unwrap().clone();

        entity.set_id(id);
        entity.set_timestamp(chrono::offset::Utc::now());
        entity.set_creation_timestamp(chrono::offset::Utc::now());
        let centity = entity.clone();

        let result = match self.entities.write().unwrap().insert(id, entity) {
            Some(_) => panic!("Entity already exists at that id"),
            None => Some(centity)
        };

        // If we are using a file, spawn a thread to write out the updated database to it
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

    // Works just like an insert
    fn update_one(&self, id: u64, mut entity: T) -> Option<T> {
        entity.set_id(id);

        println!("Entity: {:?}", entity);
        let ctimestamp = match self.entities.read().unwrap().get(&id) {
            Some(e) => e.get_creation_timestamp(),
            None => panic!("Entity does not exist")
        };
        println!("TS: {:?}", ctimestamp);
        entity.set_creation_timestamp(ctimestamp);
        entity.set_timestamp(chrono::offset::Utc::now());
        println!("Entity: {:?}", entity);
        println!("Entity: {:?}", self.entities.read().unwrap());

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
