use std::time::{Duration, SystemTime};

use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

/// JWT claims
///
/// Automatically validated when used as a handler parameter
#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: u64,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        let secret = std::env::var("JWT_SECRET")?;
        let key = DecodingKey::from_secret(secret.as_bytes());
        let token_data = decode::<Claims>(bearer.token(), &key, &Validation::default())?;

        let now = secs_from_now(0)?;
        if token_data.claims.exp < now {
            return Err(ApiError::Authentication("Expired token".to_string()));
        }

        tracing::info!("Validated claims for {}", &token_data.claims.sub);

        Ok(token_data.claims)
    }
}

pub fn create_token(sub: String) -> Result<String, ApiError> {
    let exp = secs_from_now(3600)?;
    let claims = Claims { sub, exp };

    let secret = std::env::var("JWT_SECRET")?;
    let key = EncodingKey::from_secret(secret.as_bytes());
    let token = encode(&Header::default(), &claims, &key)?;

    Ok(token)
}

pub fn secs_from_now(secs: u64) -> Result<u64, ApiError> {
    let then = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ApiError::Internal("Time went backwards".to_string()))?
        + Duration::from_secs(secs);

    Ok(then.as_secs())
}
