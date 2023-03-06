use rocket::serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;
use chrono::Utc;
use chrono::DateTime;
use database::traits::Entity;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Workflow {
    #[serde(default)]
    pub id: u64,

    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(with = "ts_seconds", default)]
    pub created_at: DateTime<Utc>,

    #[serde(with = "ts_seconds", default)]
    pub updated_at: DateTime<Utc>,
}

impl Entity for Workflow {
    fn get_id(&self) -> u64 { self.id }
    fn set_id(&mut self, id: u64) { self.id = id; }
}
