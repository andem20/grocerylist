use std::sync::Arc;

use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
#[pg_mapper(table = "lists")]
pub struct List {
    pub id: uuid::Uuid,
    pub title: String,
    pub user_id: uuid::Uuid,
}

pub struct ListRepository {
    db_pool: Arc<Pool>,
}

impl ListRepository {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self { db_pool }
    }
}

impl ListRepository {
    pub async fn find_by_user_id(&self, user_id: uuid::Uuid) -> Result<Vec<List>, DbError> {
        let client = self.client().await;
        let stmt = client
            .prepare("SELECT * FROM lists WHERE user_id = $1")
            .await?;

        let results: Vec<List> = client
            .query(&stmt, &[&user_id])
            .await?
            .iter()
            .map(|row| List::from_row_ref(&row).expect("Failed to parse list row"))
            .collect();

        Ok(results)
    }

    async fn client(&self) -> Client {
        self.db_pool.get().await.map_err(DbError::PoolError).unwrap()
    }
}
