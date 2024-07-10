use std::{
    default,
    sync::{Arc, Mutex},
};

use axum::{routing::post, Extension, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha512};

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
) -> Result<String, ApiError> {
    let mut client = client.lock().map_err(|_| ApiError::Internal)?;

    let hash = admin.secret;

    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let dbadmin = db::admin::get_by_name()
        .bind(&tx, &admin.user)
        .one()
        .await
        .map_err(|_| ApiError::WrongCredentials)?;

    let salted = format!("{}:{}", dbadmin.salt, admin.secret).as_bytes();
    let mut hasher = Sha512::new();
    hasher.update(salted);
    let hash = format!("{:X}", hasher.finalize());

    if hash != dbadmin.hash {
        Err(ApiError::WrongCredentials)
    } else {
        Ok(json!({"token": jwt::create_token(admin.user)?}))
    }
}
