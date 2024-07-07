use std::time::Duration;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
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
        match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        }
        .into_response()
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
        exp: get_exp(6000),
    };

    let secret =
        std::env::var("JWT_SECRET").expect("Environment variable JWT_SECRET is not define");

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

    (now + Duration::from_secs(seconds)).as_millis() as usize
}
