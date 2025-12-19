use anyhow::Error;
use sqlx::{SqlitePool, query_as};

use crate::domain::database::document::Document;

#[derive(Clone)]
pub struct DocumentRepository {
    pool: SqlitePool,
}

impl DocumentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_documents(&self) -> Result<Vec<Document>, Error> {
        query_as::<_, Document>(r"SELECT * FROM documents")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Error::from(e))
    }
}
