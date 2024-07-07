use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let secret =
            std::env::var("JWT_SECRET").expect("Environment variable JWT_SECRET is not defined");

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Deserialize)]
pub struct AuthPayload {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Serialize)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorMessage {
            error: String,
        }

        let (status, message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        let body = Json(ErrorMessage {
            error: message.into(),
        });

        (status, body).into_response()
    }
}

pub async fn login(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, AuthError> {
    let AuthPayload { username, password } = payload;

    if username.is_empty() || password.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    if username != "temp" || password != "temp" {
        return Err(AuthError::WrongCredentials);
    }

    let claims = Claims {
        sub: username,
        exp: get_exp(3600),
    };

    let secret =
        std::env::var("JWT_SECRET").expect("Environment variable JWT_SECRET is not defined");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthResponse { token }))
}

fn get_exp(seconds: u64) -> usize {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");

    (now + Duration::from_secs(seconds)).as_secs() as usize
}
