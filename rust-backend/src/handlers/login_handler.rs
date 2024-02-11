use actix_web::{http::Error, web, HttpResponse};
use base64::Engine;
use sha2::Digest;
use serde::Deserialize;

use crate::{repository::repository::Repository, SessionStorage};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn login(
    credentials: web::Json<Credentials>,
    repository: web::Data<Repository>,
    session: web::Data<SessionStorage>,
) -> Result<HttpResponse, Error> {
    if let Ok(user) = repository
        .users()
        .find_by_username(&credentials.username)
        .await
    {
        if credentials.password == user.password {
            let token = generate_token();
            session
                .store
                .write()
                .unwrap()
                .insert(token.clone(), user.id);
            return Ok(HttpResponse::Ok().body(token).into());
        }
    }

    Ok(HttpResponse::Unauthorized().into())
}

fn generate_token() -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(uuid::Uuid::new_v4().to_string());
    let token = hasher.finalize();
    base64::engine::general_purpose::URL_SAFE
        .encode(token)
        .to_string()
}
