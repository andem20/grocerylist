use actix::Addr;
use actix_web::{
    http::{header::AUTHORIZATION, Error},
    web, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    repository::{items_repository::Item, repository::Repository},
    websockets::{
        messages::{self, UpdateItem},
        server::WsServer,
    },
    SessionStorage,
};

#[derive(Serialize, Deserialize)]
pub struct ItemSaveRequest {
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemUpdateRequest {
    pub id: uuid::Uuid,
    pub name: String,
    pub done: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemDeleteRequest {
    pub id: uuid::Uuid,
}

pub async fn get_list_items(
    request: HttpRequest,
    path: web::Path<(uuid::Uuid,)>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
) -> Result<HttpResponse, Error> {
    let list_id = path.0;

    if let Some(token) = request.headers().get(AUTHORIZATION) {
        if let Some(user_id) = session.store.read().unwrap().get(token.to_str().unwrap()) {
            let items = repository
                .items()
                .find_by_list_id(&list_id, user_id)
                .await
                .unwrap();
            return Ok(HttpResponse::Ok().json(json!(&items)).into());
        }
    }

    return Ok(HttpResponse::Unauthorized().into());
}

pub async fn save_list_item(
    request: HttpRequest,
    body: web::Json<ItemSaveRequest>,
    path: web::Path<(uuid::Uuid,)>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
    server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let list_id = path.0;

    if let Some(token) = request.headers().get(AUTHORIZATION) {
        if let Some(_user_id) = session.store.read().unwrap().get(token.to_str().unwrap()) {
            let item = Item {
                id: uuid::Uuid::new_v4(),
                list_id,
                name: body.name.clone(),
                done: false,
            };

            let item = repository.items().save(item).await.unwrap();

            server.do_send(UpdateItem {
                action: messages::Action::CREATE,
                list_id,
                item,
            });

            return Ok(HttpResponse::Ok().into());
        }
    }

    return Ok(HttpResponse::Unauthorized().into());
}

pub async fn update_list_item(
    request: HttpRequest,
    body: web::Json<ItemUpdateRequest>,
    path: web::Path<(uuid::Uuid,)>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
    server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let list_id = path.0;

    if let Some(token) = request.headers().get(AUTHORIZATION) {
        if let Some(_user_id) = session.store.read().unwrap().get(token.to_str().unwrap()) {
            let item = Item {
                id: body.id,
                list_id,
                name: body.name.clone(),
                done: body.done,
            };

            let item = repository.items().update(item).await.unwrap();

            server.do_send(UpdateItem {
                action: messages::Action::UPDATE,
                list_id,
                item,
            });

            return Ok(HttpResponse::Ok().into());
        }
    }

    return Ok(HttpResponse::Unauthorized().into());
}

pub async fn delete_list_item(
    request: HttpRequest,
    body: web::Json<ItemDeleteRequest>,
    path: web::Path<(uuid::Uuid,)>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
    server: web::Data<Addr<WsServer>>,
) -> Result<HttpResponse, Error> {
    let list_id = path.0;

    if let Some(token) = request.headers().get(AUTHORIZATION) {
        if let Some(_user_id) = session.store.read().unwrap().get(token.to_str().unwrap()) {
            let item = Item {
                id: body.id,
                list_id,
                name: "".to_owned(),
                done: true,
            };

            let _ = repository.items().delete(body.id).await.unwrap();

            server.do_send(UpdateItem {
                action: messages::Action::DELETE,
                list_id,
                item,
            });

            return Ok(HttpResponse::Ok().into());
        }
    }

    return Ok(HttpResponse::Unauthorized().into());
}
