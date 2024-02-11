use std::{collections::HashMap, sync::RwLock};

pub mod websockets;
pub mod repository;
pub mod config;
pub mod routes;
pub mod handlers;

#[derive(Debug)]
pub struct SessionStorage {
    pub store: RwLock<HashMap<String, uuid::Uuid>>,
}

impl SessionStorage {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}