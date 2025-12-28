use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: i32,
    pub title: String,
    pub content: Value,
}
