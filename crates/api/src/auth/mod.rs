use std::sync::Arc;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use jwt::Claims;
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha512};
use tokio::sync::Mutex;

use crate::errors::ApiError;

pub mod jwt;

pub fn routes() -> Router {
    Router::new()
        .route("/auth", get(authenticate))
        .route("/auth", post(login))
}

#[derive(Deserialize)]
struct LoginForm {
    user: String,
    secret: String,
}

async fn authenticate(_claims: Claims) -> Result<StatusCode, ApiError> {
    Ok(StatusCode::OK)
}

async fn login(
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(LoginForm { user, secret }): Json<LoginForm>,
) -> Result<impl IntoResponse, ApiError> {
    if user.len() == 0 || secret.len() == 0 {
        return Err(ApiError::MissingCredentials);
    }

    let mut client = client.lock().await;
    let tx = client
        .transaction()
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let admin = db::admin::admin_by_name()
        .bind(&tx, &user)
        .one()
        .await
        .map_err(|_| ApiError::WrongCredentials)?;

    let mut hasher = Sha512::new();
    let salted = format!("{}:{}", admin.salt, secret);
    hasher.update(salted.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    if hash != admin.hash {
        Err(ApiError::WrongCredentials)
    } else {
        let token = jwt::create_token(user)?;
        Ok(Json(json!({"token": token})))
    }
}
