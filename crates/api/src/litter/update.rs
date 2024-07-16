use std::sync::Arc;

use axum::{Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError, res::ApiResponse};

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
) -> Result<ApiResponse<String>, ApiError> {
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

    Ok(ApiResponse::Success)
}
