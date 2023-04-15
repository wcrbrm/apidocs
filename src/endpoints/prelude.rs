pub use crate::endpoints::AppState;
pub use axum::extract::*;
use axum::http::StatusCode;
pub use axum::response::*;
pub use axum::Json;
pub use serde_json::Value;
pub use std::sync::Arc;
pub use tracing::*;
pub use utoipa::ToSchema;

#[derive(serde::Serialize, ToSchema)]
pub struct HttpErrMessage {
    error: String,
    message: String,
}

#[instrument(level = "warn")]
pub fn text400(
    message: impl Into<String> + std::fmt::Display + std::fmt::Debug,
) -> impl IntoResponse {
    (StatusCode::BAD_REQUEST, message.to_string()).into_response()
}

#[instrument(level = "error")]
pub fn text500(
    message: impl Into<String> + std::fmt::Display + std::fmt::Debug,
) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, message.to_string()).into_response()
}

#[instrument(level = "warn")]
pub fn err400(message: &str) -> impl IntoResponse {
    (
        StatusCode::BAD_REQUEST,
        Json(HttpErrMessage {
            error: "Bad Request".to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

#[instrument(level = "warn")]
pub fn err404(message: &str) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(HttpErrMessage {
            error: "Not Found".to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

#[instrument(level = "error")]
pub fn err500(message: &str) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(HttpErrMessage {
            error: "Server Error".to_string(),
            message: message.to_string(),
        }),
    )
        .into_response()
}

pub fn errconn() -> impl IntoResponse {
    err500("db error")
}

#[instrument(level = "warn")]
pub fn not_implemented() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(HttpErrMessage {
            error: "Error".to_string(),
            message: "Not implemented".to_string(),
        }),
    )
        .into_response()
}
