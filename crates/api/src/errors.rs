use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub enum ApiError {
    Internal,
    DatabaseError,
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Intenral server error"),
            ApiError::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            ApiError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            ApiError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            ApiError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            ApiError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        }
        .into_response()
    }
}
