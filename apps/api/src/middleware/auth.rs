use crate::AppState;
use crate::error::AppError;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::sync::Arc;
use usecase::auth::{AuthToken, Claims};

pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = Arc::from_ref(state);

        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or(AppError::MissingAuthHeader)?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AppError::InvalidAuthFormat);
        }

        // 文字列から AuthToken へ変換
        let token_str = &auth_header[7..];
        let token = AuthToken::from(token_str.to_string());

        let claims = state
            .auth_service
            .verify_token(&token)
            .map_err(|e| AppError::UseCase(e.into()))?; // verify_token returns AuthServiceError, convert to UseCaseError

        Ok(AuthenticatedUser(claims))
    }
}
