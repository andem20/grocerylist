use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum DbError {
    NotFound,
    PGError(tokio_postgres::error::Error),
    PGMError(tokio_pg_mapper::Error),
    PoolError(deadpool_postgres::PoolError),
}

impl std::error::Error for DbError {}

impl ResponseError for DbError {
    fn error_response(&self) -> HttpResponse {
        eprintln!("{:?}", *self);

        HttpResponse::InternalServerError().finish()
    }
}