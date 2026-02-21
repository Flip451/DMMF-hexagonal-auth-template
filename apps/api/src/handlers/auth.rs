use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use domain::usecase::auth::{AuthCommandUseCase, AuthQueryUseCase, LoginQuery, SignupCommand};
use domain::usecase::error::UseCaseError;
use std::sync::Arc;
use crate::AppState;

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(command): Json<SignupCommand>,
) -> impl IntoResponse {
    match state.auth_command.signup(command).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => map_usecase_error(e),
    }
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(query): Json<LoginQuery>,
) -> impl IntoResponse {
    match state.auth_query.login(query).await {
        Ok(_user) => {
            // TODO: Generate and return JWT
            (StatusCode::OK, Json(serde_json::json!({ "token": "dummy-token" }))).into_response()
        }
        Err(e) => map_usecase_error(e),
    }
}

fn map_usecase_error(error: UseCaseError) -> axum::response::Response {
    let (status, message) = match error {
        UseCaseError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
        UseCaseError::Authentication(msg) => (StatusCode::UNAUTHORIZED, msg),
        UseCaseError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        UseCaseError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        UseCaseError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        UseCaseError::Internal(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        ),
    };

    (status, Json(serde_json::json!({ "error": message }))).into_response()
}
