use axum::{Json, response::IntoResponse};
use serde::Serialize;
use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Missing authorization header")]
    MissingAuthorization,

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Asset does not exist")]
    AssetDoesNotExist,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let error_response = ErrorResponse {
            error: self.to_string(),
        };
        let status = match self {
            Self::MissingAuthorization => axum::http::StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => axum::http::StatusCode::UNAUTHORIZED,
            Self::AssetDoesNotExist => axum::http::StatusCode::NOT_FOUND,
        };
        (status, Json(error_response)).into_response()
    }
}