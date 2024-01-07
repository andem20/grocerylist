use actix::{Actor, Addr, AsyncContext, Running, StreamHandler, Handler, WrapFuture, ActorFutureExt, ActorContext, fut, dev::ContextFutureSpawner};
use actix_web::web::Data;
use actix_web_actors::ws;

use crate::websockets::server::Connect;

use super::server::{WsServer, WsMessage, Disconnect};

pub struct WsSession {
    pub id: uuid::Uuid,
    pub rooms: Vec<uuid::Uuid>,
    pub name: Option<String>,
    pub server_address: Data<Addr<WsServer>>,
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
                    Ok(_res) => (),
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

/// Handle messages from chat server, we simply send it to peer websocket
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
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => (),
        }
    }
}
