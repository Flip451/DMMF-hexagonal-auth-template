pub mod request;

use self::request::SignupRequest;
use crate::AppState;
use crate::error::AppError;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use domain::usecase::auth::AuthCommandUseCase;
use std::sync::Arc;

#[cfg_attr(feature = "openapi", utoipa::path(
    post,
    path = "/api/v1/auth/signup",
    request_body = SignupRequest,
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Invalid input"),
        (status = 409, description = "User already exists")
    ),
    tag = "auth"
))]
pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(req): Json<SignupRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.auth_command.signup(req.into()).await?;
    Ok(StatusCode::CREATED)
}
