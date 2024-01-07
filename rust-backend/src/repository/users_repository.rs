use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: String,
}

pub async fn find_all_users(client: &Client) -> Result<Vec<User>, DbError> {
    let stmt = client.prepare(&"SELECT * FROM users").await?;

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).expect("Failed to parse user row"))
        .collect::<Vec<User>>();

    Ok(results)
}

pub async fn find_user_by_id(id: uuid::Uuid, client: &Client) -> Result<User, DbError> {
    let stmt = client.prepare("SELECT * FROM users WHERE id = $1").await?;

    let row = client.query_one(&stmt, &[&id]).await?;
    let result = User::from_row_ref(&row).expect("Failed to parse user row");

    Ok(result)
}