use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde::Deserialize;

use crate::{repository::repository::Repository, websockets::{server::WsServer, session::WsSession}, SessionStorage};

#[derive(Debug, Deserialize)]
struct WsConnectRequestParams {
    token: String,
}

pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    server_address: web::Data<Addr<WsServer>>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
) -> Result<HttpResponse, Error> {
    let params = web::Query::<WsConnectRequestParams>::from_query(req.query_string()).unwrap();

    let session_token = &params.token;

    if let Some(user_id) = session.store.read().unwrap().get(session_token) {
        let lists = repository.lists().find_by_user_id(*user_id).await?;
        let ws_session = WsSession::new(*user_id, lists, None, server_address, repository);
        
        return ws::start(ws_session, &req, stream);
    }

    return Ok(HttpResponse::Unauthorized().into());
}