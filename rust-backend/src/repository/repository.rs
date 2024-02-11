use std::sync::Arc;

use deadpool_postgres::Pool;

use super::{
    items_repository::ItemRepository, lists_repository::ListRepository,
    users_repository::UserRepository,
};

pub struct Repository {
    lists: ListRepository,
    users: UserRepository,
    items: ItemRepository,
}

impl Repository {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self {
            lists: ListRepository::new(db_pool.clone()),
            users: UserRepository::new(db_pool.clone()),
            items: ItemRepository::new(db_pool.clone()),
        }
    }

    pub fn lists(&self) -> &ListRepository {
        &self.lists
    }

    pub fn users(&self) -> &UserRepository {
        &self.users
    }

    pub fn items(&self) -> &ItemRepository {
        &self.items
    }
}
