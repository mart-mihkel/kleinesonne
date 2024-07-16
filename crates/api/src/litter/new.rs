use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct NewLitter {
    name: String,
    #[serde(with = "db::BreedDef")]
    breed: db::Breed,
    parents_image: Option<String>,
    images: Vec<String>,
}

pub async fn new_litter(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(litter): Json<NewLitter>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let id = db::litter::insert_litter()
        .bind(
            &tx,
            &litter.name,
            &litter.breed,
            &litter.parents_image,
            &litter.images,
        )
        .one()
        .await?;

    tx.commit().await?;

    Ok(Json(id))
}
