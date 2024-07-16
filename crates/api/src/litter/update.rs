use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct UpdateLitter {
    id: i32,
    name: String,
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    parents_image: Option<String>,
    images: Vec<String>,
}

pub async fn update_litter(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(litter): Json<UpdateLitter>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    db::litter::update_litter()
        .bind(
            &tx,
            &litter.name,
            &litter.breed,
            &litter.parents_image,
            &litter.images,
            &litter.id,
        )
        .await?;

    tx.commit().await?;

    Ok(StatusCode::OK)
}
