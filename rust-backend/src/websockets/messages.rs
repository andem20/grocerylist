use crate::repository::{items_repository::Item, lists_repository::List};
use actix::{Message, Recipient};
use serde::{Deserialize, Serialize};

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "Vec<List>")]
pub struct Connect {
    pub id: uuid::Uuid,
    pub address: Recipient<WsMessage>,
    pub rooms: Vec<List>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: uuid::Uuid,
}

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage<T> {
    pub action: Action,
    pub resource: Resource,
    pub content: T,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct UpdateItem {
    pub action: Action,
    pub list_id: uuid::Uuid,
    pub item: Item,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    CREATE,
    READ,
    UPDATE,
    DELETE,

    CONNECT,
    DISCONNECT,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Resource {
    USER,
    LIST,
    ITEM,
    SERVER,
}
