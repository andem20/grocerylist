use std::sync::Arc;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use backend::{
    repository::{categories_repository::Category, items_repository::Item, repository::Repository},
    routes,
    services::word2vec,
    websockets::server::WsServer,
    SessionStorage,
};
use config::Config;
use deadpool_postgres::tokio_postgres::NoTls;
use dotenvy::dotenv;
use linfa::traits::Transformer;
use linfa_clustering::Dbscan;

use ndarray::Array2;
use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
struct DbConfig {
    pub pg: deadpool_postgres::Config,
}

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

    let mut items = repository.items().find_all().await.unwrap();
    let categories = repository.categories().find_all().await.unwrap();

    categorize_items(&mut items, categories);

    run_dbscan(items).into_iter().for_each(|item| {
        let rep = repository.clone();
        tokio::spawn(async move {
            let _ = rep.items().update(item).await;
        });
    });

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

fn categorize_items(items: &mut Vec<Item>, categories: Vec<Category>) {
    let w2v = word2vec::Word2Vec::new("/home/anders/Documents/projects/rust/grocery-list/rust-backend/word2vec/resources/glove-wiki-gigaword-300.w2v");

    for item in items.iter_mut() {
        let category = categories
            .iter()
            .map(|category| {
                (
                    w2v.distance(&item.name.to_lowercase(), &category.name.to_lowercase()),
                    category,
                )
            })
            .min_by(|a, b| a.0.total_cmp(&b.0))
            .map(|cat| cat.1.name.clone());

        item.set_category(category);
    }
}

fn run_dbscan(mut items: Vec<Item>) -> Vec<Item> {
    let items_array = items
        .iter()
        .map(|item| vec![item.lat.unwrap(), item.lng.unwrap()])
        .flatten()
        .collect::<Vec<f64>>();

    let observations = Array2::from_shape_vec((items_array.len() / 2, 2), items_array).unwrap();

    let min_points = 2;
    let clusters = Dbscan::params(min_points)
        .tolerance(1e-3 / 6371.0)
        .transform(&observations)
        .unwrap();

    let categories = clusters
        .to_vec()
        .iter()
        .map(|cat| cat.and_then(|x| Some(x as i16)))
        .collect::<Vec<Option<i16>>>();

    for (item, category) in items.iter_mut().zip(categories) {
        item.cluster = category;
    }

    return items;
}
