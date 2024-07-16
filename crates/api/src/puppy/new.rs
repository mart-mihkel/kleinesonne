use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{auth::jwt::Claims, errors::ApiError};

#[derive(Deserialize)]
pub struct NewPuppy {
    litter_id: i32,
    name: String,
    #[serde(with = "db::GenderDef")]
    gender: db::Gender,
    #[serde(with = "db::AvailabilityDef")]
    availability: db::Availability,
    image: Option<String>,
}

pub async fn new_puppy(
    _claims: Claims,
    Extension(client): Extension<Arc<Mutex<db::Client>>>,
    Json(puppy): Json<NewPuppy>,
) -> Result<impl IntoResponse, ApiError> {
    let mut client = client.lock().await;
    let tx = client.transaction().await?;
    let id = db::puppy::insert_puppy()
        .bind(
            &tx,
            &puppy.litter_id,
            &puppy.name,
            &puppy.gender,
            &puppy.availability,
            &puppy.image,
        )
        .one()
        .await?;

    tx.commit().await?;

    Ok(Json(id))
}
