use std::sync::Arc;

use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
#[pg_mapper(table = "items")]
pub struct Item {
    pub id: uuid::Uuid,
    pub list_id: uuid::Uuid,
    pub name: String,
    pub done: bool,
}

pub struct ItemRepository {
    db_pool: Arc<Pool>,
}

impl ItemRepository {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self { db_pool }
    }
}

impl ItemRepository {
    pub async fn find_by_list_id(
        &self,
        list_id: &uuid::Uuid,
        user_id: &uuid::Uuid,
    ) -> Result<Vec<Item>, DbError> {
        let client = self.client().await;
        let stmt = client.prepare("SELECT * FROM items WHERE list_id = $1 AND (SELECT COUNT(*) FROM lists WHERE id = $1 AND user_id = $2) = 1 ORDER BY done ASC, name DESC").await?;

        let results: Vec<Item> = client
            .query(&stmt, &[&list_id, &user_id])
            .await?
            .iter()
            .map(|row| Item::from_row_ref(&row).expect("Failed to parse item row"))
            .collect();

        Ok(results)
    }

    pub async fn save(&self, item: Item) -> Result<Item, DbError> {
        let client = self.client().await;
        let stmt = client
            .prepare("INSERT INTO items (name, list_id) VALUES ($1, $2)")
            .await?;
        client.query(&stmt, &[&item.name, &item.list_id]).await?;

        Ok(item)
    }

    pub async fn update(&self, item: Item) -> Result<Item, DbError> {
        let client = self.client().await;
        let stmt = client
            .prepare("UPDATE items SET name = $1, done = $2 WHERE id = $3")
            .await?;
        client
            .query(&stmt, &[&item.name, &item.done, &item.id])
            .await?;

        Ok(item)
    }

    pub async fn delete(&self, id: uuid::Uuid) -> Result<(), DbError> {
        let client = self.client().await;
        let stmt = client.prepare("DELETE FROM items WHERE id = $1").await?;
        client.query(&stmt, &[&id]).await?;

        Ok(())
    }

    async fn client(&self) -> Client {
        self.db_pool
            .get()
            .await
            .map_err(DbError::PoolError)
            .unwrap()
    }
}
