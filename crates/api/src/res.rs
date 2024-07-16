use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub enum ApiResponse<D: Serialize> {
    Success,
    Token(D),
    Data(D),
}

impl<D: Serialize> IntoResponse for ApiResponse<D> {
    fn into_response(self) -> Response {
        let (code, json) = match self {
            ApiResponse::Success => (StatusCode::OK, json!({"message": "success"})),
            ApiResponse::Token(t) => (StatusCode::OK, json!({"token": t})),
            ApiResponse::Data(d) => (StatusCode::OK, json!({"data": d})),
        };

        (code, Json(json)).into_response()
    }
}
