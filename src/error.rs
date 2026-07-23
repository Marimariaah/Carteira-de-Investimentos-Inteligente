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

    #[error("This username is already registered")]
    UsernameTaken,
    
    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    #[error(transparent)]
    Template(#[from] askama::Error),
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
            Self::UsernameTaken => axum::http::StatusCode::BAD_REQUEST,
            Self::InvalidCredentials => axum::http::StatusCode::UNAUTHORIZED,
            Self::AssetDoesNotExist => axum::http::StatusCode::NOT_FOUND,
            Self::MissingAuthorization => axum::http::StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) | Self::Template(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(error_response)).into_response()
    }
}