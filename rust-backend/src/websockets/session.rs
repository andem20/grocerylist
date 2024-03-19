use actix::{
    dev::ContextFutureSpawner, fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext,
    Handler, Running, StreamHandler, WrapFuture,
};
use actix_web::web::Data;
use actix_web_actors::ws;

use crate::repository::lists_repository::List;
use crate::repository::repository::Repository;

use super::messages;
use super::server::WsServer;

pub struct WsSession {
    id: uuid::Uuid,
    rooms: Vec<List>,
    name: Option<String>,
    server_address: Data<Addr<WsServer>>,
    repository: Data<Repository>,
}

impl WsSession {
    pub fn new(
        id: uuid::Uuid,
        rooms: Vec<List>,
        name: Option<String>,
        server_address: Data<Addr<WsServer>>,
        repository: Data<Repository>,
    ) -> Self {
        Self {
            id,
            rooms,
            name,
            server_address,
            repository,
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn rooms(&self) -> &Vec<List> {
        self.rooms.as_ref()
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn server_address(&self) -> &Data<Addr<WsServer>> {
        &self.server_address
    }

    pub fn repository(&self) -> Data<Repository> {
        self.repository.clone()
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let connect_msg = messages::Connect {
            id: self.id.clone(),
            address: addr.recipient(),
            rooms: self.rooms.clone(),
        };

        self.server_address
            .send(connect_msg)
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(rooms) => {
                        let server_message = messages::ClientMessage {
                            action: messages::Action::CONNECT,
                            resource: messages::Resource::SERVER,
                            content: rooms,
                        };

                        ctx.text(
                            serde_json::to_string(&server_message)
                                .unwrap_or("Failed to serialize message".to_owned()),
                        );
                    }

                    Err(e) => {
                        eprintln!("{}", e);
                        ctx.stop();
                    }
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.server_address
            .do_send(messages::Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<messages::WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: messages::WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let msg: messages::ClientMessage<String> = serde_json::from_str(&text).unwrap();

                match msg.resource {
                    messages::Resource::ITEM => match msg.action {
                        _ => println!("Action {:?} not supported", msg.action),
                    },
                    _ => println!("Type {:?} not supported", msg.resource),
                }
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
