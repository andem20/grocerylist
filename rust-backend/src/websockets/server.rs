use actix::{Actor, Handler, Recipient};
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::repository::{lists_repository::List, repository::Repository, users_repository::User};

use super::messages;

pub struct WsServer {
    rooms: HashMap<uuid::Uuid, HashSet<uuid::Uuid>>,
    sessions: HashMap<uuid::Uuid, Recipient<messages::WsMessage>>,
    repository: Arc<Repository>,
}

impl WsServer {
    pub fn new(repository: Arc<Repository>) -> Self {
        Self {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            repository,
        }
    }

    pub fn rooms(&self) -> &HashMap<uuid::Uuid, HashSet<uuid::Uuid>> {
        &self.rooms
    }

    pub fn sessions(&self) -> &HashMap<uuid::Uuid, Recipient<messages::WsMessage>> {
        &self.sessions
    }

    pub fn repository(&self) -> Arc<Repository> {
        self.repository.clone()
    }

    pub fn room_broadcast<T: Serialize>(
        &self,
        room_id: uuid::Uuid,
        content: T,
        resource: messages::Resource,
        action: messages::Action,
    ) {
        let response = messages::ClientMessage {
            action,
            resource,
            content,
        };

        self.rooms().get(&room_id).unwrap().iter().for_each(|user| {
            self.sessions()
                .get(user)
                .unwrap()
                .do_send(messages::WsMessage(
                    serde_json::to_string(&response).unwrap(),
                ));
        });
    }
}

impl Actor for WsServer {
    type Context = actix::Context<Self>;
}

impl Handler<messages::Connect> for WsServer {
    type Result = Vec<List>;

    fn handle(&mut self, msg: messages::Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_id = msg.id;

        self.sessions.insert(session_id.clone(), msg.address);

        for room in &msg.rooms {
            self.rooms
                .entry(room.id)
                .and_modify(|entry| {
                    entry.insert(session_id.clone());
                })
                .or_insert(HashSet::from([session_id]));
        }

        msg.rooms
    }
}

impl Handler<messages::Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: messages::Disconnect, _ctx: &mut Self::Context) {
        self.rooms.iter_mut().for_each(|(_, value)| {
            value.remove(&msg.id);
        });

        self.sessions.remove(&msg.id);
    }
}

impl Handler<messages::ClientMessage<Vec<User>>> for WsServer {
    type Result = ();

    fn handle(
        &mut self,
        msg: messages::ClientMessage<Vec<User>>,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.sessions().values().into_iter().for_each(|session| {
            session.do_send(messages::WsMessage(serde_json::to_string(&msg).unwrap()));
        });
    }
}

impl Handler<messages::UpdateItem> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: messages::UpdateItem, _ctx: &mut Self::Context) -> Self::Result {
        self.room_broadcast(msg.list_id, msg.item, messages::Resource::ITEM, msg.action)
    }
}
