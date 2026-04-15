use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::{Value, json};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{0}")]
    ValidationError(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Internal error: {0}")]
    InternalError(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status: StatusCode = match &self {
            AppError::DatabaseError(e) => {
                tracing::error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::InternalError(e) => {
                tracing::error!("{}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        let error: String = self.to_string().replace("\n", ", ");
        let body: Json<Value> = Json(json!({ "error": error }));

        (status, body).into_response()
    }
}
