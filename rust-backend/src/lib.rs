use std::{collections::HashMap, sync::RwLock};

pub mod config;
pub mod handlers;
pub mod repository;
pub mod routes;
pub mod services;
pub mod websockets;

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
