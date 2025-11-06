use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde_json::json;

pub enum ApiError {
    Auth(core::AuthError),
    Database(db::DbError),
    Internal(String),
}

impl From<core::AuthError> for ApiError {
    fn from(err: core::AuthError) -> Self {
        ApiError::Auth(err)
    }
}
impl From<db::DbError> for ApiError {
    fn from(err: db::DbError) -> Self {
        ApiError::Database(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self)->Response {
        let (status, message) = match self {
            ApiError::Auth(core::AuthError::InvalidCredentials) => {
                (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())
            }
            ApiError::Auth(core::AuthError::UserAlreadyExists) => {
                (StatusCode::CONFLICT, "User already exists".to_string())
            }
            ApiError::Auth(_) => (StatusCode::UNAUTHORIZED, "Authentication failed".to_string()),
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()),
            ApiError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}