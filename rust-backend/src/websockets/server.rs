use std::{collections::{HashMap, HashSet}, sync::Arc};
use actix::{Actor, Handler, Recipient, Message};
use deadpool_postgres::{Pool, Client};
use serde::{Serialize, Deserialize};

use crate::{repository::{lists_repository::List, items_repository::Item}, config::postgres_config::DbError};

pub struct WsServer {
    rooms: HashMap<uuid::Uuid, HashSet<uuid::Uuid>>,
    sessions: HashMap<uuid::Uuid, Recipient<WsMessage>>,
    db_pool: Arc<Pool>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);


#[derive(Message)]
#[rtype(result = "Vec<List>")]
pub struct Connect {
    pub id: uuid::Uuid,
    pub address: Recipient<WsMessage>,
    pub rooms: Vec<List>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: uuid::Uuid,
}

#[derive(Message, Debug, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub action: ClientAction,
    pub content_type: String,
    pub content: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub enum ClientAction {
    CREATE,
    READ,
    UPDATE,
    DELETE
}

#[derive(Message, Debug, Serialize)]
#[rtype(result = "()")]
pub struct ServerMessage<T> {
    pub content_type: String,
    pub content: T
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemsRequest {
    pub list_id: uuid::Uuid,
    pub user_id: uuid::Uuid
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemSaveRequest {
    pub list_id: uuid::Uuid,
    pub name: String
}

impl WsServer {
    pub fn new(db_pool: Arc<Pool>) -> Self {
        Self {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
            db_pool
        }
    }
}

impl Actor for WsServer {
    type Context = actix::Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = Vec<List>;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
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

        println!("{:?}", self.rooms);
        
        msg.rooms
    }
}

impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) {
        self.rooms.iter_mut().for_each(|(_, value)| {
            value.remove(&msg.id);
        });

        println!("{:?}", self.rooms);

        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _ctx: &mut Self::Context) {
        match msg.content_type.as_str() {
            "Items" => {
                match msg.action {
                    ClientAction::READ => {
                        if let Some(content) = msg.content {
                            let request: ItemsRequest = serde_json::from_str(&content).expect("Failed to deserialize request");
                            
                            let pool = self.db_pool.clone();
                            let addr = self.sessions.get(&request.user_id).unwrap().clone();
                            
                            tokio::spawn(async move {
                                let client: Client = pool.get().await.map_err(DbError::PoolError).unwrap();
                                let items = Item::find_by_list_id(request.list_id.clone(), &client).await.unwrap();
                                let response = ServerMessage {
                                    content_type: "ITEMS_RESPONSE".to_string(),
                                    content: items
                                };
                                
                                addr.do_send(WsMessage(serde_json::to_string(&response).unwrap()))
                            });
                        }
                    },

                    ClientAction::CREATE => {
                        if let Some(content) = msg.content {
                            let pool = self.db_pool.clone();
                            let rooms = self.rooms.clone();
                            let sessions = self.sessions.clone();

                            tokio::spawn(async move {
                                let request: ItemSaveRequest = serde_json::from_str(&content).expect("Failed to deserialize request");
                                let item = Item {
                                    id: uuid::Uuid::new_v4(),
                                    list_id: request.list_id,
                                    name: request.name,
                                    done: false,
                                };

                                let client: Client = pool.get().await.map_err(DbError::PoolError).unwrap();
                                let saved_item = item.save(&client).await;

                                match saved_item {
                                    Ok(item) => {
                                        let response = ServerMessage {
                                            content_type: "ITEM_CREATE_RESPONSE".to_string(),
                                            content: item
                                        };

                                        rooms.get(&response.content.list_id)
                                            .unwrap()
                                            .iter().for_each(|user| {
                                                sessions.get(user).unwrap()
                                                    .do_send(WsMessage(serde_json::to_string(&response).unwrap()));
                                            });
                                    },
                                    Err(e) => eprintln!("{:?}", e) 
                                }
                            });
                        }
                    },

                    _ => println!("Action {:?} not supported", msg.action)
                }
            },
            _ => println!("Type {} not supported", msg.content_type)
        }
    }
}