use std::time::{Duration, SystemTime};

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, Header, Validation};
use serde::Deserialize;

use crate::errors::ApiError;

#[derive(Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = std::env::var("JWT_SECRET").map_err(|_| ApiError::Internal)?;

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::InvalidToken)?;

        let token_data = decode::<Claims>(bearer.token(), &secret, &Validation::default())
            .map_err(|_| ApiError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

pub fn create_token(sub: String) -> Result<String, ApiError> {
    let secret = std::env::var("JWT_SECRET").map_err(|_| ApiError::Internal)?;

    let exp = secs_from_now(3600);
    let claims = Claims { sub, exp };

    let token = encode(&Header::default(), &claims, &secret).map_err(|_| ApiError::Internal)?;

    Ok(token)
}

fn secs_from_now(secs: u64) -> u64 {
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    (now + Duration::from_secs(secs)).as_secs()
}
