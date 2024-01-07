use std::collections::{HashMap, HashSet};
use actix::{Actor, Handler, Message, Recipient};

pub struct WsServer {
    rooms: HashMap<uuid::Uuid, HashSet<uuid::Uuid>>,
    sessions: HashMap<uuid::Uuid, Recipient<WsMessage>>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);


#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub id: uuid::Uuid,
    pub address: Recipient<WsMessage>,
    pub rooms: Vec<uuid::Uuid>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: uuid::Uuid,
}


impl WsServer {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            sessions: HashMap::new()
        }
    }
}

impl Actor for WsServer {
    type Context = actix::Context<Self>;
}

impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let session_id = msg.id;

        self.sessions.insert(session_id.clone(), msg.address);

        for room in msg.rooms {
            self.rooms
                .entry(room)
                .and_modify(|entry| { 
                    entry.insert(session_id.clone()); 
                })
                .or_insert(HashSet::from([session_id]));
        }

        println!("{:?}", self.rooms);
        
        0
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