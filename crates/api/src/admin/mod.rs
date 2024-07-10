use std::sync::Arc;

use axum::{response::IntoResponse, routing::post, Extension, Json, Router};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha512};
use tokio::sync::Mutex;

use crate::errors::ApiError;

mod jwt;

pub fn routes() -> Router {
    Router::new().route("/auth", post(auth))
}

#[derive(Deserialize)]
struct AdminForm {
    user: String,
    secret: String,
}

async fn auth(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(admin): Json<AdminForm>,
) -> Result<impl IntoResponse, ApiError> {
    let AdminForm { user, secret } = admin;

    if user.len() == 0 || secret.len() == 0 {
        return Err(ApiError::MissingCredentials);
    }

    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dbadmin = db::admin::get_by_name()
        .bind(&tx, &user)
        .one()
        .await
        .map_err(|_| ApiError::WrongCredentials)?;

    let salted = format!("{}:{}", dbadmin.salt, secret);
    let mut hasher = Sha512::new();
    hasher.update(salted.as_bytes());
    let hash = format!("{:X}", hasher.finalize());

    if hash != dbadmin.hash {
        Err(ApiError::WrongCredentials)
    } else {
        Ok(Json(json!({"jwt": jwt::create_token(user)?})))
    }
}
