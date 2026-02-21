pub mod request;
pub mod response;

use self::request::LoginRequest;
use self::response::LoginResponse;
use crate::AppState;
use crate::error::AppError;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use domain::usecase::auth::AuthQueryUseCase;
use std::sync::Arc;

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    ),
    tag = "auth"
)]
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(req): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response_dto = state.auth_query.login(req.into()).await?;
    Ok((StatusCode::OK, Json(LoginResponse::from(response_dto))))
}
