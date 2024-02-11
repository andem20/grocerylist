use crate::handlers::{list_handler, login_handler, ws_handler};
use actix_web::web::{self, delete, get, post, put, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    let auth_scope =
        web::scope("/auth").service(web::resource("/login").route(post().to(login_handler::login)));

    let ws_scope =
        web::scope("/ws").service(web::resource("").route(get().to(ws_handler::websocket)));

    let list_scope = web::scope("/list").service(
        web::resource("/{list_id}/items")
            .route(get().to(list_handler::get_list_items))
            .route(post().to(list_handler::save_list_item))
            .route(put().to(list_handler::update_list_item))
            .route(delete().to(list_handler::delete_list_item)),
    );

    cfg.service(auth_scope)
        .service(ws_scope)
        .service(list_scope);
}
