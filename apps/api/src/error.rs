use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::usecase::error::UseCaseError;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    /// UseCase 層で発生したエラー
    UseCase(UseCaseError),
    /// 認証ヘッダーが欠落している
    MissingAuthHeader,
    /// 認証ヘッダーの形式が不正
    InvalidAuthFormat,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::UseCase(usecase_error) => return AppError::map_usecase_error(usecase_error),
            AppError::MissingAuthHeader => (
                StatusCode::UNAUTHORIZED,
                "Missing authorization header".to_string(),
            ),
            AppError::InvalidAuthFormat => (
                StatusCode::UNAUTHORIZED,
                "Invalid authorization format".to_string(),
            ),
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "type": status.canonical_reason().unwrap_or("Unknown"),
            }
        }));

        (status, body).into_response()
    }
}

impl AppError {
    fn map_usecase_error(error: UseCaseError) -> Response {
        let (status, message) = match &error {
            UseCaseError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            UseCaseError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            UseCaseError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            UseCaseError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            UseCaseError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            UseCaseError::Internal(err) => {
                tracing::error!(error = ?err, "Internal server error occurred");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "type": status.canonical_reason().unwrap_or("Unknown"),
            }
        }));

        (status, body).into_response()
    }
}

impl From<UseCaseError> for AppError {
    fn from(inner: UseCaseError) -> Self {
        Self::UseCase(inner)
    }
}
