use thiserror::Error;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Token has expired")]
    TokenExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Access denied: insufficient permissions")]
    Forbidden,
}
