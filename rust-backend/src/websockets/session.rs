use actix::{Actor, Addr, AsyncContext, Running, StreamHandler, Handler, WrapFuture, ActorFutureExt, ActorContext, fut, dev::ContextFutureSpawner};
use actix_web::web::Data;
use actix_web_actors::ws;
use deadpool_postgres::Pool;

use crate::{websockets::server::Connect, repository::lists_repository::List};

use super::server::{WsServer, WsMessage, Disconnect, ClientMessage, ServerMessage};

pub struct WsSession {
    pub id: uuid::Uuid,
    pub rooms: Vec<List>,
    pub name: Option<String>,
    pub server_address: Data<Addr<WsServer>>,
    pub db_pool: Data<Pool>
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let connect_msg = Connect { 
            id: self.id.clone(), 
            address: addr.recipient(),
            rooms: self.rooms.clone()
        };

        self.server_address
            .send(connect_msg)
            .into_actor(self)
            .then(|res, _act, ctx| {
                match res {
                    Ok(rooms) => {
                        let server_message: ServerMessage<Vec<List>> = ServerMessage {
                            content_type: "CONNECT_RESPONSE".to_owned(),
                            content: rooms,
                        };

                        ctx.text(serde_json::to_string(&server_message).unwrap_or("Failed to serialize message".to_owned()));
                    },

                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        self.server_address.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let client_message: ClientMessage = serde_json::from_str(&text).expect("Failed to deserialize string");
                self.server_address.do_send(client_message);
            },
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}