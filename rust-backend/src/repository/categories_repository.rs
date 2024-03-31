use std::sync::Arc;

use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: uuid::Uuid,
    pub name: String,
}

impl Category {
    pub fn new(id: uuid::Uuid, name: String) -> Self {
        Self { id, name }
    }
}

pub struct CategoryRepository {
    db_pool: Arc<Pool>,
}

impl CategoryRepository {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self { db_pool }
    }
}

impl CategoryRepository {
    pub async fn find_all(&self) -> Result<Vec<Category>, DbError> {
        let client = self.client().await;

        let query = "SELECT * FROM categories";

        let stmt = client.prepare(query).await?;

        let results: Vec<Category> = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| Category::from_row_ref(&row).expect("Failed to parse item row"))
            .collect();

        Ok(results)
    }

    async fn client(&self) -> Client {
        self.db_pool
            .get()
            .await
            .map_err(DbError::PoolError)
            .unwrap()
    }
}
