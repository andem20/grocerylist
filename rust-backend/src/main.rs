use std::{sync::Arc, str::FromStr};

use actix::{Actor, Addr};
use actix_cors::Cors;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use backend::{websockets::{server::WsServer, session::WsSession}, repository::{users_repository::{find_all_users, find_user_by_id}, lists_repository::find_lists_by_user_id}, config::postgres_config::DbError};
use config::Config;
use deadpool_postgres::{tokio_postgres::NoTls, Client, Pool};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct WsConnectRequestParams {
    session_id: String
}

async fn index(req: HttpRequest, stream: web::Payload, server_address: web::Data<Addr<WsServer>>, db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let params = web::Query::<WsConnectRequestParams>::from_query(req.query_string()).unwrap();

    let user_id = uuid::Uuid::from_str(&params.session_id).unwrap();

    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let user = find_user_by_id(user_id, &client).await?;
    let lists = find_lists_by_user_id(user_id, &client)
        .await?
        .iter()
        .map(|e| e.id)
        .collect();

    println!("{:?}", user);

    let session = WsSession {
        id: user_id,
        rooms: lists,
        name: None,
        server_address,
    };

    return ws::start(session, &req, stream);
}

#[derive(Debug, Default, Deserialize)]
struct DbConfig {
    pub pg: deadpool_postgres::Config,
}

pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DbError::PoolError)?;
    let users = find_all_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: DbConfig = config.try_deserialize().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = Arc::new(WsServer::new().start());

    println!("Server running on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::from(server.clone()))
            .wrap(cors)
            .route("/users", web::get().to(get_users))
            .route("/ws", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}