use axum::{response::IntoResponse, http::StatusCode};

pub mod user;

pub enum ApiError {
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}