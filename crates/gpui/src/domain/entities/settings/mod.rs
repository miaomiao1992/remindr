use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};

#[derive(Serialize, Deserialize)]
pub enum DbContext {
    Local(LocalDatabase),
    Unknown,
}

impl DbContext {
    pub fn parse(value: Value) -> DbContext {
        let database_type = from_value::<PartialDatabase>(value.clone())
            .unwrap()
            .database_type;

        match database_type {
            DatabaseType::Local => DbContext::Local(from_value::<LocalDatabase>(value).unwrap()),
            _ => DbContext::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum DatabaseType {
    Local,
    Remote,
}

#[derive(Serialize, Deserialize)]
struct PartialDatabase {
    #[serde(rename = "type")]
    pub database_type: DatabaseType,
}

#[derive(Serialize, Deserialize)]
pub struct LocalDatabase {
    pub name: String,
    pub path: String,
}
