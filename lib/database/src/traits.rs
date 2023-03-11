use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};


pub trait Entity {
    fn get_id(&self) -> u64;
    fn set_id(&mut self, id: u64);
}

pub trait Database<T: Sized + Entity> {
    fn insert_one(&self, entity: T) -> Option<T>;
    fn read(&self, id: u64) -> Option<T>;
    fn read_all(&self) -> Vec<T>;
    fn update_one(&self, id: u64, entity: T) -> Option<T>;
    fn delete(&self, id: u64) -> Option<T>;
}

pub trait Indexable<T> { 
    fn add_index(&self, entity: &T);
    fn search(&self, index: &str, value: IndexType) -> Vec<T>;
}

pub trait Index {
    fn get_indexes(&self) -> Vec<(&str, IndexType)>;
}

pub trait Timestamp {
    fn get_creation_timestamp(&self) -> DateTime<Utc>;
    fn set_creation_timestamp(&mut self, timestamp: DateTime<Utc>);
    fn set_timestamp(&mut self, timestamp: DateTime<Utc>);
}

pub trait IndexableDatabase<T: Sized + Entity>: Database<T> + Indexable<T> {
    fn insert(&self, entity: T) -> Option<T>;
    fn update(&self, id: u64, entity: T) -> Option<T>;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum IndexType 
where 
    String: Ord,
    u64: Sized + Ord,
    u32: Sized + Ord,
    u16: Sized + Ord,
    i64: Sized + Ord,
    i32: Sized + Ord,
    i16: Sized + Ord,
{
    String (String),
    u64 (u64),
    u32 (u32),
    u16 (u16),
    i64 (i64),
    i32 (i32),
    i16 (i16),
}

impl PartialEq for IndexType
where
    String: Ord + Eq,
    u64: Sized + Ord + Eq,
    u32: Sized + Ord + Eq,
    u16: Sized + Ord + Eq,
    i64: Sized + Ord + Eq,
    i32: Sized + Ord + Eq,
    i16: Sized + Ord + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::u64(l0), Self::u64(r0)) => l0 == r0,
            (Self::u32(l0), Self::u32(r0)) => l0 == r0,
            (Self::u16(l0), Self::u16(r0)) => l0 == r0,
            (Self::i64(l0), Self::i64(r0)) => l0 == r0,
            (Self::i32(l0), Self::i32(r0)) => l0 == r0,
            (Self::i16(l0), Self::i16(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for IndexType
where 
    String: Ord + Eq,
    u64: Sized + Ord + Eq,
    u32: Sized + Ord + Eq,
    u16: Sized + Ord + Eq,
    i64: Sized + Ord + Eq,
    i32: Sized + Ord + Eq,
    i16: Sized + Ord + Eq,
{

}

impl PartialOrd for IndexType
where 
    String: Ord + Eq,
    u64: Sized + Ord + Eq,
    u32: Sized + Ord + Eq,
    u16: Sized + Ord + Eq,
    i64: Sized + Ord + Eq,
    i32: Sized + Ord + Eq,
    i16: Sized + Ord + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl Ord for IndexType
where 
    String: Ord,
    u64: Sized + Ord,
    u32: Sized + Ord,
    u16: Sized + Ord,
    i64: Sized + Ord,
    i32: Sized + Ord,
    i16: Sized + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            IndexType::String(s) => {
                match other {
                    IndexType::String(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::u64(s) => {
                match other {
                    IndexType::u64(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::u32(s) => {
                match other {
                    IndexType::u32(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::u16(s) => {
                match other {
                    IndexType::u16(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::i64(s) => {
                match other {
                    IndexType::i64(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::i32(s) => {
                match other {
                    IndexType::i32(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
            IndexType::i16(s) => {
                match other {
                    IndexType::i16(o) => s.cmp(o),
                    _ => std::cmp::Ordering::Less
                }
            },
        }
    }
}
