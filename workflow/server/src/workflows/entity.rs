use serde::{Serialize, Deserialize};
use chrono::serde::ts_seconds;
use chrono::Utc;
use chrono::DateTime;
use database_derive::Database;

#[derive(Database, Serialize, Deserialize, Clone, Debug)]
pub struct Workflow {
    #[serde(default)]
    #[id]
    pub id: u64,

    #[index]
    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub yaml: String,

    #[creation_timestamp]
    #[serde(with = "ts_seconds", default)]
    pub created_at: DateTime<Utc>,

    #[timestamp]
    #[serde(with = "ts_seconds", default)]
    pub updated_at: DateTime<Utc>,
}
