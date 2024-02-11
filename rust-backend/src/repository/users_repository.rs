use std::sync::Arc;

use deadpool_postgres::{Client, Pool};
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::config::postgres_config::DbError;

#[derive(Deserialize, PostgresMapper, Serialize, Debug)]
#[pg_mapper(table = "users")]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub roles: String,
}

pub struct UserRepository {
    db_pool: Arc<Pool>,
}

impl UserRepository {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self { db_pool }
    }
}

impl UserRepository {
    pub async fn find_all(&self) -> Result<Vec<User>, DbError> {
        let client = self.client().await;
        let stmt = client.prepare(&"SELECT * FROM users").await?;
    
        let results = client
            .query(&stmt, &[])
            .await?
            .iter()
            .map(|row| User::from_row_ref(row).expect("Failed to parse user row"))
            .collect::<Vec<User>>();
    
        Ok(results)
    }
    
    pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<User, DbError> {
        let client = self.client().await;
        let stmt = client.prepare("SELECT * FROM users WHERE id = $1").await?;
    
        let row = client.query_one(&stmt, &[&id]).await?;
        let result = User::from_row_ref(&row).expect("Failed to parse user row");
    
        Ok(result)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, DbError> {
        let client = self.client().await;
        let stmt = client.prepare("SELECT * FROM users WHERE username = $1").await?;
    
        let row = client.query_one(&stmt, &[&username]).await?;
        let result = User::from_row_ref(&row).expect("Failed to parse user row");
    
        Ok(result)
    }

    async fn client(&self) -> Client {
        self.db_pool.get().await.map_err(DbError::PoolError).unwrap()
    }
}