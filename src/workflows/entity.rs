use rocket::serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;
use chrono::Utc;
use chrono::DateTime;
use crate::database_traits::Entity;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workflow {
    #[serde(skip)]
    pub id: i32,

    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(with = "ts_seconds", default)]
    pub created_at: DateTime<Utc>,

    #[serde(with = "ts_seconds", default)]
    pub updated_at: DateTime<Utc>,
}

impl Entity for Workflow {
    fn get_id(&self) -> i32 { self.id }
    fn set_id(&mut self, id: i32) { self.id = id; }
}
