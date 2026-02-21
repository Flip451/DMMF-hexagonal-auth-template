use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::AuthenticatedUser;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use domain::models::user::UserIdentity;
use domain::usecase::auth::{AuthCommandUseCase, AuthQueryUseCase, LoginQuery, SignupCommand};
use std::sync::Arc;

pub async fn signup(
    State(state): State<Arc<AppState>>,
    Json(command): Json<SignupCommand>,
) -> Result<impl IntoResponse, AppError> {
    state.auth_command.signup(command).await?;
    Ok(StatusCode::CREATED)
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(query): Json<LoginQuery>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.auth_query.login(query).await?;

    // JWT 発行
    let token = state
        .auth_service
        .issue_token(user.id())
        .map_err(|e| AppError::UseCase(e.into()))?;

    Ok((StatusCode::OK, Json(serde_json::json!({ "token": token }))))
}

pub async fn me(AuthenticatedUser(claims): AuthenticatedUser) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({ "user_id": claims.sub })),
    )
}
