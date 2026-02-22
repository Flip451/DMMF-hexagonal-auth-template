use crate::models::auth::PasswordServiceError;
use crate::usecase::auth::AuthServiceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Access denied: insufficient permissions")]
    Forbidden,

    #[error("Password service failure")]
    PasswordService(#[from] PasswordServiceError),

    #[error("Auth service failure")]
    AuthService(#[from] AuthServiceError),
}
