use crate::AppState;
use crate::error::AppError;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::sync::Arc;

pub struct AuthenticatedUser(pub domain::models::auth::Claims);

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

        let token = &auth_header[7..];
        let claims = state
            .auth_service
            .verify_token(token)
            .map_err(|e| AppError::UseCase(e.into()))?;

        Ok(AuthenticatedUser(claims))
    }
}
