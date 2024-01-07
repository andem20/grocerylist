use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "lists")]
pub struct List {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

pub async fn find_lists_by_user_id(user_id: uuid::Uuid, client: &Client) -> Result<Vec<List>, DbError> {
    let stmt = client.prepare("SELECT * FROM lists WHERE user_id = $1").await?;

    let results: Vec<List> = client.query(&stmt, &[&user_id])
        .await?
        .iter()
        .map(|row| List::from_row_ref(&row).expect("Failed to parse list row"))
        .collect();

    Ok(results)
}