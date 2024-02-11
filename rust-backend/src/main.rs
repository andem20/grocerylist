use std::sync::Arc;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{
    middleware::Logger, web, App, HttpServer,
};
use backend::{repository::repository::Repository, routes, websockets::server::WsServer, SessionStorage};
use config::Config;
use deadpool_postgres::tokio_postgres::NoTls;
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct DbConfig {
    pub pg: deadpool_postgres::Config,
}

// #[get("/users")]
// async fn get_users(
//     repository: web::Data<Repository>,
//     server_addr: web::Data<Addr<WsServer>>
// ) -> Result<HttpResponse, Error> {
//     let users = repository.users().find_all().await?;

//     let response = messages::ClientMessage {
//         action: messages::Action::READ,
//         resource: messages::Resource::USER,
//         content: users,
//     };

//     server_addr.do_send(response);

//     Ok(HttpResponse::Ok().into())
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: DbConfig = config.try_deserialize().unwrap();
    let pool = Arc::new(config.pg.create_pool(None, NoTls).unwrap());
    let repository = Arc::new(Repository::new(pool));
    let server = web::Data::new(WsServer::new(repository.clone()).start());
    let session = web::Data::new(SessionStorage::new());

    println!("Server running on http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::from(repository.clone()))
            .app_data(server.clone())
            .app_data(session.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .configure(routes::routes)
    })
    .bind(("127.0.0.1", 8080))?
    .bind(("192.168.123.31", 8080))?
    .run()
    .await
}
