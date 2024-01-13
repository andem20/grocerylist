use deadpool_postgres::Client;
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

impl Item {
    pub async fn find_by_list_id(list_id: uuid::Uuid, client: &Client) -> Result<Vec<Item>, DbError> {
        let stmt = client.prepare("SELECT * FROM items WHERE list_id = $1").await?;
    
        let results: Vec<Item> = client.query(&stmt, &[&list_id])
            .await?
            .iter()
            .map(|row| Item::from_row_ref(&row).expect("Failed to parse item row"))
            .collect();
    
        Ok(results)
    }

    pub async fn save(self, client: &Client) -> Result<Item, DbError> {
        let stmt = client.prepare("INSERT INTO items (name, list_id) VALUES ($1, $2)").await?;
        client.query(&stmt, &[&self.name, &self.list_id]).await?;

        Ok(self)
    }
}
