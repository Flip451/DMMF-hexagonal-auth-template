use thiserror::Error;

use crate::models::auth::PasswordServiceError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Access denied: insufficient permissions")]
    Forbidden,

    #[error("Password service failure")]
    PasswordService(#[from] PasswordServiceError),
}
